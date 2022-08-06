fn avg_of_slice(data: &[u32]) -> f64 {
    let mut sum = 0f64;
    let mut count = 0;

    for i in data {
        sum += *i as f64;
        count += 1;
    }

    sum / (count as f64)
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let avg = avg_of_slice(&data);
    println!("average: {}", avg);
}
