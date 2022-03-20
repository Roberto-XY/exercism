pub fn is_armstrong_number(num: u32) -> bool {
    let (digit_count, iter) = digits(num);

    let armstrong_sum: u32 = iter.map(|x| x.pow(digit_count)).sum();

    if armstrong_sum == num {
        true
    } else {
        false
    }
}

fn digits(mut num: u32) -> (u32, impl Iterator<Item = u32>) {
    let mut divisor = 1;
    let mut digit_count = 1;
    while num >= divisor * 10 {
        divisor *= 10;
        digit_count += 1;
    }

    let iter = std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    });

    (digit_count, iter)
}
