fn is_prime(n: i32) -> bool {

    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i32;

    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let test_data = [
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (1000, false),
        ];

        for (n, exp) in test_data {
            assert_eq!(is_prime(n), exp, "n = {}", n);
        }
    }
}

fn main() {
    let numbers = [2, 3, 4, 5, 6, 7, 1000];
    for &num in numbers.iter() {
        println!("{} is prime? {}", num, is_prime(num));
    }
}
