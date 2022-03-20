use std::ptr::NonNull;

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
            let checked_collatz_step = step_val.checked_mul(3).and_then(|x| x.checked_add(1));

            if let Some(c) = checked_collatz_step {
                step_val = c;
                step += 1;
            } else {
                return None;
            }
        }
    }

    Some(step)
}
