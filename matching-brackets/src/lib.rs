const S: [char; 3] = ['(', '[', '{'];
const E: [char; 3] = [')', ']', '}'];

fn is_start(ch: &char) -> bool {
    return "[({".find(*ch) != None
}
fn is_end(ch: &char) -> bool {
    return "])}".find(*ch) != None
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<usize> = vec![];

    for c in string.chars() {
        if is_start(&c) {
            stack.push(S.iter().position(|&s| s == c).unwrap());
        } else if is_end(&c) {
            // check if last one is match
            let last = stack.pop();
            if last != E.iter().position(|&s| s == c) {
               return false;
            }
        }
    }

    if stack.len() == 0 {
        return true;
    }

    false
}
