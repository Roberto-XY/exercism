use std::iter;

pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars_iter = source.chars().peekable();
    let mut curr_count = 0;

    while let Some(c) = chars_iter.next() {
        curr_count += 1;
        if chars_iter.peek() != Some(&c) {
            if curr_count > 1 {
                result.push_str(&curr_count.to_string())
            }
            result.push(c);
            curr_count = 0;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut chars_iter = source.chars();

    iter::from_fn(|| match chars_iter.next() {
        None => None,
        Some(c) if c.is_numeric() => {
            let mut current_c = c;
            let mut number_str = String::new();

            while let true = current_c.is_numeric() {
                number_str.push(current_c);
                current_c = chars_iter.next().unwrap();
            }

            let num = number_str.parse::<usize>().unwrap();
            Some(iter::repeat(current_c).take(num))
        }
        Some(c) => Some(iter::repeat(c).take(1)),
    })
    .fuse()
    .flatten()
    .collect()
}
