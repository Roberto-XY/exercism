use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .try_fold(HashSet::new(), |mut acc, c| match acc.get(&c) {
            None => {
                acc.insert(c);
                Some(acc)
            }
            Some(_) => None,
        })
        .map_or_else(|| false, |_| true)
}
