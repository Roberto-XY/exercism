pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if is_question(message) && is_yelled(message) {
        "Calm down, I know what I'm doing!"
    } else if is_question(message) {
        "Sure."
    } else if is_yelled(message) {
        "Whoa, chill out!"
    } else if message.is_empty() {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}

fn is_yelled(message: &str) -> bool {
    let is_alphabetic = message.chars().any(|x| x.is_alphabetic());

    message.to_uppercase() == message && is_alphabetic
}
