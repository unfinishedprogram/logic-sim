use std::{collections::VecDeque, time::Duration};

pub struct Stopwatch {
    last: std::time::Instant,
    samples: VecDeque<std::time::Duration>,
    max_samples: usize,
}

impl Stopwatch {
    pub fn new(max_samples: usize) -> Self {
        Self {
            last: std::time::Instant::now(),
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn running_average(&self) -> std::time::Duration {
        let sum: std::time::Duration = self.samples.iter().sum();
        if self.samples.is_empty() {
            Duration::ZERO
        } else {
            sum / self.samples.len() as u32
        }
    }

    pub fn start(&mut self) {
        self.last = std::time::Instant::now();
    }

    pub fn end(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now - self.last;
        self.last = now;
        self.samples.push_front(elapsed);
        if self.samples.len() > self.max_samples {
            self.samples.pop_back();
        }
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new(60)
    }
}
