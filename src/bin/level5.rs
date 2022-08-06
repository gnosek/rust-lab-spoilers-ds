use rand::Rng;

#[derive(Debug)]
struct Stats {
    sum: f64,
    count: usize,
    min: u32,
    max: u32,
}

impl Stats {
    fn update(&mut self, sample: u32) {
        self.sum += sample as f64;
        self.count += 1;
        if self.min > sample {
            self.min = sample;
        }
        if self.max < sample {
            self.max = sample;
        }
    }

    fn avg(&self) -> Option<f64> {
        match self.count {
            0 => None,
            c => Some(self.sum / c as f64),
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            sum: 0.0,
            count: 0,
            min: u32::MAX,
            max: u32::MIN,
        }
    }
}

fn slice_stats<F: FnMut(u32)>(data: &[u32], mut f: F) {
    for sample in data {
        f(*sample)
    }
}

fn main() {
    let mut data = Vec::new();
    data.resize(100, 0);

    let mut rng = rand::thread_rng();
    rng.fill(data.as_mut_slice());

    let mut stats = Stats::default();

    slice_stats(&data, |sample| stats.update(sample));

    println!("Stats: {:?}", stats);

    match stats.avg() {
        Some(avg) => println!("average: {}", avg),
        None => println!("Cannot get average of empty vector"),
    }
}
