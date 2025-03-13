use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_idx = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx = i;
        }
    }
    (min_sum, min_idx, min_idx + 1)
}

fn print_adjacent_pairs(data: &[i32]) {
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        println!(
            "indexes: {},{}   data: {},{}   sum={}",
            i,
            i + 1,
            data[i],
            data[i + 1],
            sum
        );
    }
}

fn main() {

    let data = gen_random_vector(20);

    print_adjacent_pairs(&data);

    let (min_sum, idx1, idx2) = min_adjacent_sum(&data);
    println!(
        "min adjacent sum={}={}+{} at indexes={},{}",
        min_sum, data[idx1], data[idx2], idx1, idx2
    );
}
