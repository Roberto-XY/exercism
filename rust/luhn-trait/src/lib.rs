pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let acc_opt = self
            .to_string()
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .try_fold((0u32, 0u32), |(sum, digit_counter), (idx, digit_opt)| {
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
            });

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
