pub fn series2(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        vec!["".to_string(); digits.len() + 1]
    } else {
        digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|x| x.iter().collect())
            .collect()
    }
}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len() + 1 - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
