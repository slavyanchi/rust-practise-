fn invert_the_case(input: String) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_the_case() {
        let data = ["Hello", "HELLO", "Power", "MiXed"];
        let expected = ["hELLO", "hello", "pOWER", "mIxED"];

        for (input, exp) in data.iter().zip(expected.iter()) {
            assert_eq!(invert_the_case(input.to_string()), *exp);
        }
    }
}

fn main() {
    let input = String::from("Hello Rust!");
    let inverted = invert_the_case(input);
    println!("{}", inverted); 
}
