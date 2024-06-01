use std::collections::VecDeque;

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

    pub fn tick(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now - self.last;
        self.last = now;
        self.samples.push_front(elapsed);
        if self.samples.len() > self.max_samples {
            self.samples.pop_back();
        }
    }

    pub fn running_average(&self) -> std::time::Duration {
        let sum: std::time::Duration = self.samples.iter().sum();
        sum / self.samples.len() as u32
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new(60)
    }
}
