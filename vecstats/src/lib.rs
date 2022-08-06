use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::BTreeMap;

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

    fn combine(&mut self, other: &Self) {
        self.sum += other.sum;
        self.count += other.count;
        if self.min > other.min {
            self.min = other.min;
        }
        if self.max < other.max {
            self.max = other.max
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

#[pyfunction]
fn vec_stats(py: Python, data: Vec<u32>) -> BTreeMap<&'static str, PyObject> {
    let totals = data
        .par_iter()
        .fold(Stats::default, |mut stats, &sample| {
            stats.update(sample);
            stats
        })
        .reduce(Stats::default, |mut a, b| {
            a.combine(&b);
            a
        });

    let mut stats = BTreeMap::new();
    stats.insert("count", totals.count.to_object(py));
    if totals.count != 0 {
        stats.insert("sum", totals.sum.to_object(py));
        stats.insert("min", totals.min.to_object(py));
        stats.insert("max", totals.max.to_object(py));
        stats.insert("avg", totals.avg().to_object(py));
    }
    stats
}

/// A Python module implemented in Rust.
#[pymodule]
fn vecstats(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(vec_stats, m)?)?;
    Ok(())
}
