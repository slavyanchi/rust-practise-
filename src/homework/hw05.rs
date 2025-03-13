
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd() {

        let data = vec![
            ((6, 12), 6),
            ((8, 20), 4),
            ((10, 80), 10),
            ((54, 24), 6),
            ((18, 12), 6),
        ];
        for ((a, b), expected) in data {
            assert_eq!(
                gcd(a, b),
                expected,

                a,
                b,
                expected,
                gcd(a, b)
            );
        }
    }
}
