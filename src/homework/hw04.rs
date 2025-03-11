const WIDTH: usize = 9;   
const HEIGHT: usize = 9;  

fn main() {
    let mut output = String::new(); 
    let center = HEIGHT / 2;       

    for row in 0..HEIGHT {
        let star_count = if row <= center {
            2 * row + 1
        } else {
            2 * (HEIGHT - 1 - row) + 1
        };

        let space_count = (WIDTH - star_count) / 2; 
      
        for _ in 0..space_count {
            output.push(' ');
        }

        for _ in 0..star_count {
            output.push('*');
        }

        output.push('\n');
    }

    print!("{}", output);
    println!();
}
