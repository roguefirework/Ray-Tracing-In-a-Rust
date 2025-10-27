#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn from(a:&Interval,b:&Interval) -> Interval {
        Interval::new(
            if a.min < b.min { a.min } else { b.min },
            if a.max > b.max { a.max } else { b.max }
        )
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn empty() -> Interval {
        Interval::new(f64::INFINITY, f64::NEG_INFINITY)
    }
    pub fn universe() -> Interval {Interval::new(f64::NEG_INFINITY, f64::INFINITY)}
    pub fn expand(&self, delta:f64) -> Interval {Interval::new(self.min - delta/2.0, self.max - delta/2.0)}
    pub fn clamp(&self, x : f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            return self.max;
        } else {
            return x;
        }
    }
}