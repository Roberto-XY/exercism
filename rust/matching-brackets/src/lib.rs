pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}

use std::ops::ControlFlow;

pub fn brackets_are_balanced2(string: &str) -> bool {
    let result = string.chars().try_fold(Vec::new(), |mut stack, c| match c {
        '(' | '{' | '[' => {
            stack.push(c);
            ControlFlow::Continue(stack)
        }
        ')' => {
            if stack.pop() != Some('(') {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(stack)
            }
        }
        ']' => {
            if stack.pop() != Some('[') {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(stack)
            }
        }
        '}' => {
            if stack.pop() != Some('{') {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(stack)
            }
        }
        _ => ControlFlow::Continue(stack),
    });

    match result {
        ControlFlow::Continue(stack) => stack.is_empty(),
        ControlFlow::Break(_) => false,
    }
}
