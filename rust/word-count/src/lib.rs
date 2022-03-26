use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| c.is_ascii_whitespace() || (c.is_ascii_punctuation() && c != '\''))
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('\'').to_ascii_lowercase())
        .fold(HashMap::new(), |mut acc, s| {
            let counter = acc.entry(s).or_insert(0);
            *counter += 1;
            acc
        })
}
