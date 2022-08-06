use rand::Rng;

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
    let mut data = Vec::new();
    data.resize(100, 0);

    let mut rng = rand::thread_rng();
    rng.fill(data.as_mut_slice());

    match avg_of_slice(&data) {
        Some(avg) => println!("average: {}", avg),
        None => println!("Cannot get average of empty vector"),
    }
}
