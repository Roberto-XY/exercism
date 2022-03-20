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

    if acc.is_empty() {
        n.to_string()
    } else {
        acc
    }
}

pub fn raindrops2(n: u32) -> String {
    let outputs = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(i, _)| n % i == 0)
        .map(|(_, s)| *s)
        .collect::<String>();

    if outputs.is_empty() {
        outputs
    } else {
        n.to_string()
    }
}
