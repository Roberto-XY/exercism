pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|&x| is_prime(x))
        .nth((n + 1) as usize)
        .unwrap()
}

fn is_prime(n: u32) -> bool {
    let sqrt_limit = (n as f64).sqrt() as u32 + 1;

    !(2..sqrt_limit).any(|i| n % i == 0)
}
