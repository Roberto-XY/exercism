use std::iter;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let number = number
        .iter()
        .copied()
        .skip_while(|&x| x == 0)
        .collect::<Vec<_>>();

    if from_base == 1 || from_base == 0 {
        return Err(Error::InvalidInputBase);
    }
    if to_base == 1 || to_base == 0 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }

    let (number_base10, _): (u32, _) =
        number
            .iter()
            .try_fold((0, number.len() as u32), |(acc, exp), &x| {
                if x >= from_base {
                    Err(Error::InvalidDigit(x))
                } else {
                    Ok((acc + x * from_base.pow(exp - 1), exp - 1))
                }
            })?;

    Ok(horner_schema(number_base10, to_base))
}

fn horner_schema(number: u32, to_base: u32) -> Vec<u32> {
    let mut curr_number = number;
    let mut curr_exponent = 0u32;

    let mut res = iter::from_fn(|| {
        if curr_number == 0 {
            None
        } else {
            let next = Some(curr_number % to_base);

            curr_exponent += 1;
            curr_number /= to_base;

            next
        }
    })
    .fuse()
    .collect::<Vec<_>>();

    res.reverse();

    res
}
