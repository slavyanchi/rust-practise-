use std::io;

fn main() {
    println!("Enter the number of triangles:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num_triangles: usize = input.trim().parse().expect("Please enter a valid number");

    let max_width = 4 * num_triangles + 1;
  
    for i in 1..=num_triangles {

        let height = 2 * i + 1;

        for row in 0..height {

            let asterisks = 2 * row + 1;

            let spaces = (max_width - asterisks) / 2;

            println!("{}{}", " ".repeat(spaces), "*".repeat(asterisks));
        }
    }
}
