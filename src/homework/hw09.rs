fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {

        return s;
    }
    
    let shift = n.rem_euclid(len as isize) as usize;
    
    if shift == 0 {

        return s;
    }

    let split_point = len - shift;
    let (left, right) = s.split_at(split_point);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefg";
        let shifts = [
            ( 0, "abcdefg"),
            ( 1, "gabcdef"),
            ( 2, "fgabcde"),
            ( 3, "efgabcd"),
            ( 4, "defgabc"),
            ( 5, "cdefgab"),
            ( 6, "bcdefga"),
            ( 7, "abcdefg"),
            ( 8, "gabcdef"),
            (-1, "bcdefga"),
            (-2, "cdefgab"),
            (-3, "defgabc"),
            (-4, "efgabcd"),
            (-5, "fgabcde"),
            (-6, "gabcdef"),
            (-7, "abcdefg"),
            (-8, "bcdefga"),
        ];

        for (n, expected) in shifts {
            assert_eq!(
                rotate(s.to_string(), n),
                expected,
                "rotate(\"{}\", {}) should be \"{}\"",
                s,
                n,
                expected
            );
        }
    }
}

fn main() {
    let original = "abcdefg".to_string();
    let rotated = rotate(original.clone(), 3);
    println!("Original: {}", original);
    println!("Rotated by 3: {}", rotated);
}
