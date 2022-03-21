pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut step = 0;
    let mut step_val = n;

    while step_val != 1 {
        if step_val % 2 == 0 {
            step_val = step_val / 2;
            step += 1;
        } else {
            step_val = step_val.checked_mul(3)?.checked_add(1)?;
            step += 1;
        }
    }

    Some(step)
}

pub fn collatz2(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz2(n / 2).map(|x| x + 1),
        n => collatz2(n.checked_mul(3)?.checked_add(1)?).map(|x| x + 1),
    }
}

pub fn collatz_fail(n: u64) -> Option<u64> {
    collatz_rec(n, 0)
}

fn collatz_rec(n: u64, step: u64) -> Option<u64> {
    if n == 1 {
        Some(step)
    } else {
        if n % 2 == 0 {
            collatz_rec(n / 2, step + 1)
        } else {
            let checked_collatz_step = n.checked_mul(3).and_then(|x| x.checked_add(1));

            if let Some(c) = checked_collatz_step {
                collatz_rec(c, step + 1)
            } else {
                None
            }
        }
    }
}
