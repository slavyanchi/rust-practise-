use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> i64 {
    let n = shipments.len();
    if n == 0 {

        return 0;
    }

    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1;
    }

    let avg = total as usize / n; 
    let mut moves = 0usize;
    
    for &cargo in shipments {
        if cargo as usize > avg {
            moves += cargo as usize - avg;
        }
    }

    moves as i64
}

#[allow(dead_code)]
fn count_permutation_option(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len();
    if n == 0 {
        return Some(0);
    }

    let total: u32 = shipments.iter().sum();
    if total as usize % n != 0 {
        return None;
    }

    let avg = total as usize / n;
    let mut moves = 0usize;
    for &cargo in shipments {
        if cargo as usize > avg {
            moves += cargo as usize - avg;
        }
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut data: Vec<u32> = (0..n).map(|_| rng.gen_range(1..=10)).collect();
    let sum: u32 = data.iter().sum();

    let remainder = sum % (n as u32);
    if remainder != 0 {
        let last = data.last_mut().unwrap();

        if *last >= remainder {
            *last -= remainder;
        } else {
            *last += (n as u32 - remainder);
        }
    }
    data
}

fn main() {

    let example1 = vec![8, 2, 2, 4, 4];
    let moves1 = count_permutation(&example1);
    println!("{:?} => moves = {}", example1, moves1);

    let example2 = vec![9, 3, 7, 2, 9];
    let moves2 = count_permutation(&example2);
    println!("{:?} => moves = {}", example2, moves2);

    let n = 5;
    let generated = gen_shipments(n);
    let moves_generated = count_permutation(&generated);
    println!(
        "Generated: {:?}\nSum = {}, moves = {}",
        generated,
        generated.iter().sum::<u32>(),
        moves_generated
    );
}
