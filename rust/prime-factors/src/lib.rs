pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut candidate = n;
    let mut factor = 2;

    while candidate > 1 {
        if candidate % factor == 0 {
            res.push(factor);
            candidate /= factor
        } else {
            factor += 1;
        }
    }

    res
}
