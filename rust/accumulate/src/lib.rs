/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut acc = Vec::with_capacity(input.len());

    for item in input {
        acc.push(function(item));
    }

    acc
}
