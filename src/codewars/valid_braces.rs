pub fn valid_braces(s: &str) -> bool {
    let left: [char; 3] = ['(', '[', '{'];
    let right: [char; 3] = [')', ']', '}'];
    let mut stack: Vec<char> = Vec::new();
    for character in s.chars() {
        if left.contains(&character) {
            stack.push(character);
        } else if right.contains(&character) {
            let pos = right.iter().position(|&r| r == character).unwrap();
            if stack.len() > 0 && (left[pos] == *stack.last().unwrap()) {
                stack.pop();
            } else {
                return false;
            }
        }
    }
    if stack.len() == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn match_valid_braces(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '{' => stack.push(c),
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            '[' => stack.push(c),
            ']' => {
                if stack.pop() != Some(']') {
                    return false;
                }
            }
            _ => panic!("Invalid Input"),
        }
    }
    stack.is_empty()
}

pub fn clever_valid_braces(s: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => {
                if Some(ch) != stack.pop() {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}
