use std::ops::Range;

#[derive(Debug)]
pub struct Interval {
    range: Range<f64>,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { range: min..max }
    }

    pub fn min(&self) -> f64 {
        self.range.start
    }

    pub fn max(&self) -> f64 {
        self.range.end
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.range.start < x && x < self.range.end
    }

    pub fn clamp(&self, x: f64) -> f64 {
        match x {
            x if x < self.range.start => self.range.start,
            x if x > self.range.end => self.range.end,
            _ => x,
        }
    }
}
