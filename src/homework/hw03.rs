const WIDTH: usize = 20; 
const HEIGHT: usize = 20; 

fn main() {
    let mut output = String::new(); 

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 {
                output.push('*');
            } else if i == j || i + j == WIDTH - 1 {
                output.push('*');
            } else {
                output.push(' '); 
            }
        }
        output.push('\n'); 
    }

    print!("{}", output); 
}
