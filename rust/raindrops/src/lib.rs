pub fn raindrops(n: u32) -> String {
    let mut acc = String::new();

    if n % 3 == 0 {
        acc.push_str("Pling")
    }
    if n % 5 == 0 {
        acc.push_str("Plang")
    }
    if n % 7 == 0 {
        acc.push_str("Plong")
    }

    if acc == String::new() {
        n.to_string()
    } else {
        acc
    }
}
