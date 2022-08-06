fn avg_of_slice(data: &[u32]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }

    let mut sum = 0f64;
    let mut count = 0;

    for i in data {
        sum += *i as f64;
        count += 1;
    }

    Some(sum / (count as f64))
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    match avg_of_slice(&data) {
        Some(avg) => println!("average: {}", avg),
        None => println!("Cannot get average of empty vector"),
    }
}
