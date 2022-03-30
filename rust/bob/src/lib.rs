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
    let is_any_alphabetic = message.chars().any(|x| x.is_alphabetic());
    let is_all_uppercase = message
        .chars()
        .all(|x| !x.is_alphabetic() || x.is_uppercase());

    is_all_uppercase && is_any_alphabetic
}
