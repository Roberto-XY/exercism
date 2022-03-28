pub struct Luhn {
    vec: Vec<Option<u32>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let acc_opt = self.vec.iter().rev().enumerate().try_fold(
            (0u32, 0u32),
            |(sum, digit_counter), (idx, digit_opt)| {
                digit_opt.map(|digit| {
                    let res = if (idx + 1) % 2 == 0 {
                        let doubled = digit * 2;
                        if doubled > 9 {
                            doubled - 9
                        } else {
                            doubled
                        }
                    } else {
                        digit
                    };

                    (res + sum, digit_counter + 1)
                })
            },
        );

        if let Some((sum, counter)) = acc_opt {
            if counter > 1 {
                sum % 10 == 0
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        let vec = input
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| c.to_digit(10))
            .collect();

        Luhn { vec }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}

impl From<u8> for Luhn {
    fn from(input: u8) -> Self {
        let vec = input.to_string().chars().map(|c| c.to_digit(10)).collect();
        Luhn { vec }
    }
}

impl From<u16> for Luhn {
    fn from(input: u16) -> Self {
        let vec = input.to_string().chars().map(|c| c.to_digit(10)).collect();
        Luhn { vec }
    }
}

impl From<u32> for Luhn {
    fn from(input: u32) -> Self {
        let vec = input.to_string().chars().map(|c| c.to_digit(10)).collect();
        Luhn { vec }
    }
}

impl From<u64> for Luhn {
    fn from(input: u64) -> Self {
        let vec = input.to_string().chars().map(|c| c.to_digit(10)).collect();
        Luhn { vec }
    }
}

impl From<usize> for Luhn {
    fn from(input: usize) -> Self {
        let vec = input.to_string().chars().map(|c| c.to_digit(10)).collect();
        Luhn { vec }
    }
}
