use std::marker::PhantomData;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

pub struct LinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    len: usize,
}

struct Node<T> {
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }
}

pub struct Cursor<'a, T: 'a> {
    index: usize,
    current: Option<*mut Node<T>>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    head: Option<*mut Node<T>>,
    len: usize,
    marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            index: 0,
            current: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            index: self.len.saturating_sub(1),
            current: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            marker: PhantomData,
        }
    }

    fn unlink_node(&mut self, node: *mut Node<T>) {
        let node = unsafe { &mut *node };

        match node.prev {
            Some(prev) => unsafe { (*prev).next = node.next },
            None => self.head = node.next,
        };

        match node.next {
            Some(next) => unsafe { (*next).prev = node.prev },
            None => self.tail = node.prev,
        };

        self.len -= 1;
    }

    unsafe fn splice_nodes(
        &mut self,
        existing_prev: Option<*mut Node<T>>,
        existing_next: Option<*mut Node<T>>,
        mut splice_start: *mut Node<T>,
        mut splice_end: *mut Node<T>,
        splice_length: usize,
    ) {
        if let Some(mut existing_prev) = existing_prev {
            (*existing_prev).next = Some(splice_start);
        } else {
            self.head = Some(splice_start);
        }
        if let Some(mut existing_next) = existing_next {
            (*existing_next).prev = Some(splice_end);
        } else {
            self.tail = Some(splice_end);
        }

        (*splice_start).prev = existing_prev;
        (*splice_end).next = existing_next;

        self.len += splice_length;
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.current.map(|next| &mut (*next).element) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = match self.current.take() {
                None => {
                    let next = self.list.head;
                    self.current = next;
                    self.index = 0;
                    dbg!(next)
                }

                Some(current) => {
                    let next = (*current).next;
                    self.current = next;
                    self.index += 1;
                    dbg!(next)
                }
            };
            next.map(|next| &mut (*next).element)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = match self.current.take() {
                None => {
                    self.index = 0;
                    None
                }

                Some(current) => {
                    let prev = (*current).prev;
                    self.current = prev;
                    self.index -= 1;
                    prev
                }
            };
            prev.map(|prev| &mut (*prev).element)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let removed_node = self.current?;
        unsafe {
            if self.index < self.list.len - 1 {
                self.current = (*removed_node).next;
                self.list.unlink_node(removed_node);
                let removed_node = Box::from_raw(removed_node);
                Some(removed_node.element)
            } else {
                self.current = (*removed_node).prev;
                self.list.unlink_node(removed_node);
                let removed_node = Box::from_raw(removed_node);
                Some(removed_node.element)
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            let spliced_node = Box::leak(Box::new(Node::new(element)));
            let node_next = match self.current {
                None => self.list.head,
                Some(node) => (*node).next,
            };
            self.list
                .splice_nodes(self.current, node_next, spliced_node, spliced_node, 1);
            if self.current.is_none() {
                self.index = self.list.len;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            let spliced_node = Box::leak(Box::new(Node::new(element)));
            let node_prev = match self.current {
                None => self.list.tail,
                Some(node) => (*node).prev,
            };
            self.list
                .splice_nodes(node_prev, self.current, spliced_node, spliced_node, 1);
            self.index += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|next_node| unsafe {
                let next_node = &*next_node;
                self.len -= 1;
                self.head = next_node.next;
                &next_node.element
            })
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.cursor_front().take() {
            drop(node);
        }
    }
}
