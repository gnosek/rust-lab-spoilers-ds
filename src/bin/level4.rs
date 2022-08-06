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

fn main() {
    let mut data = Vec::new();
    data.resize(100, 0);

    let mut rng = rand::thread_rng();
    rng.fill(data.as_mut_slice());

    let mut stats = Stats::default();

    for sample in &data {
        stats.update(*sample);
    }

    println!("Stats: {:?}", stats);

    match stats.avg() {
        Some(avg) => println!("average: {}", avg),
        None => println!("Cannot get average of empty vector"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let data = Vec::new();
        let mut stats = Stats::default();

        for sample in &data {
            stats.update(*sample);
        }

        assert_eq!(stats.sum, 0.0);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.min, u32::MAX);
        assert_eq!(stats.max, u32::MIN);
        assert!(stats.avg().is_none());
    }

    #[test]
    fn test_not_empty() {
        let data = vec![1, 2, 3, 6];
        let mut stats = Stats::default();

        for sample in &data {
            stats.update(*sample);
        }

        assert_eq!(stats.sum, 12.0);
        assert_eq!(stats.count, 4);
        assert_eq!(stats.min, 1);
        assert_eq!(stats.max, 6);
        assert_eq!(stats.avg(), Some(3.0));
    }
}
