pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&x| factors.iter().any(|&factor| factor != 0 && x % factor == 0))
        .sum()
}
