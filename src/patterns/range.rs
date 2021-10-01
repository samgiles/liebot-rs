use serde::{Serialize, Deserialize};
use rand::distributions::uniform::SampleUniform;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Range<T: PartialOrd + Copy> {
    upper: T,
    lower: T,
}

impl Range<f32> {
    pub fn new() -> Self {
        Self { upper: f32::MIN, lower: f32::MAX }
    }
    pub fn into_json<R: rand::Rng>(&self, rng: &mut R) -> serde_json::Value {
        let val = self.select(rng);

        serde_json::Value::Number(serde_json::Number::from_f64(val as f64).unwrap())
    }
}

impl Range<f64> {
    pub fn new() -> Self {
        Self { upper: f64::MIN, lower: f64::MAX }
    }
    pub fn into_json<R: rand::Rng>(&self, rng: &mut R) -> serde_json::Value {
        let val = self.select(rng);

        serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap())
    }
}

impl Range<i32> {
    pub fn new() -> Self {
        Self { upper: i32::MIN, lower: i32::MAX }
    }
    pub fn into_json<R: rand::Rng>(&self, rng: &mut R) -> serde_json::Value {
        let val = self.select(rng);

        serde_json::Value::Number(serde_json::Number::from(val))
    }
}

impl Range<i64> {
    pub fn new() -> Self {
        Self { upper: i64::MIN, lower: i64::MAX }
    }
    pub fn into_json<R: rand::Rng>(&self, rng: &mut R) -> serde_json::Value {
        let val = self.select(rng);

        serde_json::Value::Number(serde_json::Number::from(val))
    }
}

impl Range<u64> {
    pub fn new() -> Self {
        Self { upper: u64::MIN, lower: u64::MAX }
    }
    
    pub fn into_json<R: rand::Rng>(&self, rng: &mut R) -> serde_json::Value {
        let val = self.select(rng);

        serde_json::Value::Number(serde_json::Number::from(val))
    }
}

impl<T: PartialOrd + Copy + SampleUniform> Range<T> { 
    pub fn update(&mut self, value: T) {
        if value > self.upper {
            self.upper = value;
        }

        if value < self.lower {
            self.lower = value;
        }
    }

    pub fn merge(&mut self, range: &Range<T>) {
        self.update(range.upper);
        self.update(range.lower);
    }

    pub fn select<R: rand::Rng>(&self, rng: &mut R) -> T {
        rng.gen_range(self.lower..=self.upper)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_range() {
        let mut range = Range::<f32>::new();
        range.update(100.0);
        range.update(-100.0);

        assert_eq!(range.upper, 100.0);
        assert_eq!(range.lower, -100.0);
        
        range.update(200.0);

        assert_eq!(range.upper, 200.0);
    }
}
