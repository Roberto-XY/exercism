use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(head) => list
            .windows(2)
            .map(|x| format!("For want of a {} the {} was lost.\n", x[0], x[1]))
            .chain(once(format!("And all for the want of a {0}.", head)))
            .collect(),
    }
}
