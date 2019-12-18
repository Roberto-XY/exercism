pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let mut index = 0;

    loop {
        if string[index..].starts_with('{') {
            stack.push('{');
            index += 1;
        } else if string[index..].starts_with('[') {
            stack.push('[');
            index += 1;
        } else if string[index..].starts_with('(') {
            stack.push('(');
            index += 1;
        } else if string[index..].starts_with('}') {
            match stack.pop() {
                Some('{') => index += 1,
                Some(_unexpected) => break false,
                None => break false,
            }
        } else if string[index..].starts_with(']') {
            match stack.pop() {
                Some('[') => index += 1,
                Some(_unexpected) => break false,
                None => break false,
            }
        } else if string[index..].starts_with(')') {
            match stack.pop() {
                Some(')') => index += 1,
                Some(_unexpected) => break false,
                None => break false,
            }
        } else if index == string.len() && stack.is_empty() {
            break true;
        } else if index < string.len() {
            index += 1;
        } else {
            break false;
        }
    }
}
