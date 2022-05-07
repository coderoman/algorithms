#[allow(dead_code)]

pub fn balanced_brackets(str: &str) -> bool {
    let mut stack: Vec<u8> = Vec::new();
    const BRACES: [u8; 3] = [b'{', b'(', b'['];

    str.as_bytes().iter().all(|b| match b {
        b'}' => stack.pop() == Some(b'{'),
        b')' => stack.pop() == Some(b'('),
        b']' => stack.pop() == Some(b'['),
        _ => {
            if BRACES.contains(b) {
                stack.push(*b);
            }
            true
        }
    }) && stack.is_empty()
}
mod tests {
    use super::*;

    #[test]
    fn are_brackets_balanced() {
        assert!(balanced_brackets("(((())))()[]{}"))
    }

    #[test]
    fn curly_brackets_balanced() {
        assert!(balanced_brackets("{}"))
    }

    #[test]
    fn square_brackets_balanced() {
        assert!(balanced_brackets("[]"))
    }

    #[test]
    fn are_brackets_unbalanced() {
        assert!(!balanced_brackets("(((()))"))
    }
}
