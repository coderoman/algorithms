#[allow(dead_code)]

fn prefix_length(a: &str, b: &str) -> u8 {
    if a.is_empty() {
        return 0;
    }

    let mut prefix: u8 = 0;
    for (index, char1) in a.as_bytes().iter().enumerate() {
        if let Some(char2) = b.as_bytes().get(index) {
            if char1 == char2 {
                prefix += 1;
                continue;
            }
            break;
        }
    }

    prefix
}

fn postfix_length(a: &str, b: &str) -> u8 {
    if a.is_empty() {
        return 0;
    }

    let mut postfix: u8 = 0;
    for (index, char1) in a.as_bytes().iter().rev().enumerate() {
        if let Some(char2) = b.as_bytes().get(index) {
            if char1 == char2 {
                postfix += 1;
                continue;
            }
            break;
        }
    }

    postfix
}

fn calculate_levenshtein_distance(mut a: &'static str, mut b: &'static str) -> bool {
    if a == b {
        return true;
    }

    if a.len().abs_diff(b.len()) > 1 {
        return false;
    }

    let max_prefix = prefix_length(a, b);
    let max_postfix = postfix_length(a, b);

    if a.len() > b.len() {
        (a, b) = (b, a)
    }

    max_prefix + max_postfix >= b.len() as u8 - 1
}

mod tests {
    use super::*;

    #[test]
    fn strings_are_empty() {
        assert!(calculate_levenshtein_distance("", ""));
    }

    #[test]
    fn order_is_not_important() {
        assert_eq!(
            calculate_levenshtein_distance("a", ""),
            calculate_levenshtein_distance("", "a")
        );
    }

    #[test]
    fn same_prefix_different_strings() {
        assert!(!calculate_levenshtein_distance("asd", "asdfda"));
    }

    #[test]
    fn different_strings_lengths() {
        assert!(!calculate_levenshtein_distance("a", "bb"));
    }

    #[test]
    fn tail_adding() {
        assert!(calculate_levenshtein_distance("a", "ab"));
    }

    #[test]
    fn almost_the_same() {
        assert!(!calculate_levenshtein_distance("abc", "bcd"));
    }

    #[test]
    fn long_strings() {
        assert!(!calculate_levenshtein_distance("abbac", "cabab"));
    }
}
