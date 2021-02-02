const VALID: &[char] = &['[', ']', '(', ')', '{', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let is_valid = |&x: &char| VALID.contains(&x);

    !string.chars().any(|c| {
        if !is_valid(&c) {
            return false;
        }
        string.matches(c).count() % 2 != 0
    })
}
