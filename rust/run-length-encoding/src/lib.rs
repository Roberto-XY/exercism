#[derive(Debug)]
enum EncodingState {
    Uninitialized,
    Running(char, u64),
}

pub fn encode(source: &str) -> String {
    let (mut acc, remaining_state) = source.chars().fold(
        (String::new(), EncodingState::Uninitialized),
        |(mut acc, encoding_state), c| match encoding_state {
            EncodingState::Uninitialized => (acc, EncodingState::Running(c, 1)),

            EncodingState::Running(last_char, counter) if last_char == c => {
                (acc, EncodingState::Running(c, counter + 1))
            }

            EncodingState::Running(last_char, 1) => {
                acc.push(last_char);
                (acc, EncodingState::Running(c, 1))
            }

            EncodingState::Running(last_char, counter) => {
                acc.extend(counter.to_string().chars());
                acc.push(last_char);
                (acc, EncodingState::Running(c, 1))
            }
        },
    );

    match remaining_state {
        EncodingState::Uninitialized => acc,

        EncodingState::Running(last_char, 1) => {
            acc.push(last_char);
            acc
        }

        EncodingState::Running(last_char, counter) => {
            acc.extend(counter.to_string().chars());
            acc.push(last_char);
            acc
        }
    }
}

pub fn decode(source: &str) -> String {
    let mut chars_iter = source.chars();

    std::iter::from_fn(|| match chars_iter.next() {
        None => None,
        Some(c) if c.is_numeric() => {
            let mut current_c = c;
            let mut number_str = String::new();

            while let true = current_c.is_numeric() {
                number_str.push(current_c);
                current_c = chars_iter.next().unwrap();
            }

            let num = number_str.parse::<usize>().unwrap();
            Some(vec![current_c; num])
        }
        Some(c) => Some(vec![c]),
    })
    .flat_map(|vec| vec.into_iter())
    .fuse()
    .collect()
}
