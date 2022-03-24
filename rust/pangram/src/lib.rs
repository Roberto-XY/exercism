use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let unique_letter_count = sentence
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<char>>()
        .len();
    let alphabet_count = 26;

    unique_letter_count == alphabet_count
}
