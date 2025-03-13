fn is_palindrome(x: i32) -> bool {

    if x < 0 {
        return false;
    }

    let s = x.to_string();

    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1001, true),
            (-121, false),
            (0, true),
        ];

        for (val, expected) in data {
            assert_eq!(
                is_palindrome(val),
                expected,
                "Expected is_palindrome({}) to be {}",
                val,
                expected
            );
        }
    }
}

fn main() {
    let numbers = [123, 121, 1001, -121, 0];
    for &num in &numbers {
        println!("{} is palindrome? {}", num, is_palindrome(num));
    }
}
