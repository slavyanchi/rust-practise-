use itertools::Itertools;

fn main() {
  
    let solutions = solve_muxa_slon();

    for (m, u, x, a, s, l, o, n) in &solutions {
        let muxa = format!("{}{}{}{}", m, u, x, a);
        let slon = format!("{}{}{}{}", s, l, o, n);

        println!("  {}", muxa);
        println!("x         {}", a);
        println!("----------");
        println!("  {}", slon);
        println!();
    }

    println!("Total solutions: {}", solutions.len());
}

fn solve_muxa_slon() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let digits = 1..=8;
    let mut solutions = Vec::new();

    for (m, u, x, a, s, l, o, n) in digits.permutations(8) {
        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;
      
        if muxa * a == slon {
            solutions.push((m, u, x, a, s, l, o, n));
        }
    }

    solutions
}
