use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    ops::Deref,
    ptr::NonNull, cell::Cell,
};

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```

// InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId {
    id: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId {
    id: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId {
    id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Clone, Debug, PartialEq)]
struct InputCell<T> {
    value: T,
    dependants: HashSet<ComputeCellId>,
}

impl<T> InputCell<T> {
    fn new(value: T) -> Self {
        InputCell {
            value,
            dependants: HashSet::new(),
        }
    }
}

struct ComputeCell<'a, T> {
    value: T,
    f: Box<dyn Fn(&[T]) -> T + 'a>,
    args: Vec<CellId>,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>,
    dependants: HashSet<ComputeCellId>,
}

impl<'a, T> ComputeCell<'a, T> {
    fn new(value: T, f: Box<dyn Fn(&[T]) -> T>, args: Vec<CellId>) -> Self {
        ComputeCell {
            value,
            f,
            args,
            callbacks: HashMap::new(),
            dependants: HashSet::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, InputCell<T>>,
    compute_cells: Cell<HashMap<ComputeCellId, ComputeCell<'a, T>>>,
    last_callback_id: usize,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: Cell::new(HashMap::new()),
            last_callback_id: 0,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId {
            id: self.input_cells.len(),
        };
        self.input_cells.insert(id, InputCell::new(initial));
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let compute_id = ComputeCellId {
            id: self.compute_cells.len(),
        };

        let non_existent_dep = dependencies.iter().find(|x| match x {
            CellId::Compute(id) => !self.compute_cells.contains_key(&id),
            CellId::Input(id) => !self.input_cells.contains_key(&id),
        });

        if let Some(&id) = non_existent_dep {
            return Err(id);
        }

        self.add_dependants(compute_id, dependencies);

        let value = self.compute_function(&compute_func, &dependencies).unwrap();
        let cell = ComputeCell::new(value, Box::new(compute_func), dependencies.to_vec());
        self.compute_cells.insert(compute_id, cell);

        Ok(compute_id)
    }

    fn add_dependants(&mut self, dependant: ComputeCellId, dependencies: &[CellId]) {
        dependencies.iter().for_each(|cell| match cell {
            CellId::Input(id) => {
                let cell = self.input_cells.get_mut(id).unwrap();
                cell.dependants.insert(dependant);
            }
            CellId::Compute(id) => {
                let cell = self.compute_cells.get_mut(id).unwrap();
                cell.dependants.insert(dependant);
            }
        });
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.input_cells.get(&id).map(|x| x.value),
            CellId::Compute(id) => self.compute_cells.get(&id).map(|x| x.value),
        }
    }

    fn compute_function(&self, f: &impl Fn(&[T]) -> T, args: &[CellId]) -> Option<T> {
        args.iter()
            .map(|&cell_id| self.value(cell_id))
            .collect::<Option<Vec<T>>>()
            .map(|v| f(&v))
    }

    fn compute<'b>(&self, compute_cell: &'b mut ComputeCell<'a, T>) -> T
    where
        T: Debug,
    {
        let args = compute_cell
            .args
            .iter()
            .map(|&cell_id| match cell_id {
                CellId::Input(id) => self.input_cells.get(&id).map(|x| x.value).unwrap(),
                CellId::Compute(id) => self
                    .compute_cells
                    .get_mut(&id)
                    .map(|x| self.compute(x))
                    .unwrap(),
            })
            .collect::<Vec<T>>();

        let value = compute_cell.f.deref()(args.deref());

        dbg!(&value);

        if compute_cell.value != value {
            for callback in compute_cell.callbacks.values_mut() {
                callback(value);
            }
            compute_cell.value = value;
        };
        value
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool
    where
        T: Debug,
    {
        if let Some(input_cell) = self.input_cells.get_mut(&id) {
            input_cell.value = new_value;

            dbg!(&new_value);
            dbg!(&input_cell.dependants);

            for compute_cell_id in input_cell.dependants.clone().iter() {
                let compute_cell = self.compute_cells.get(compute_cell_id).unwrap();
            }

            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Some(cell) = self.compute_cells.get_mut(&id) {
            self.last_callback_id += 1;

            let callback_id = CallbackId {
                id: self.last_callback_id,
            };

            cell.callbacks.insert(callback_id, Box::new(callback));
            Some(callback_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cell) = self.compute_cells.get_mut(&cell) {
            if let Some(_) = cell.callbacks.remove(&callback) {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
