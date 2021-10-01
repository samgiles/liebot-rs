use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoolPattern {
    total_values_observed: u32,
    total_true_values_observed: u32,
}

impl BoolPattern {
    pub fn new(value: bool) -> Self {
        Self {
            total_values_observed: 1,
            total_true_values_observed: if value { 1 } else  { 0 },
        }
    }
    pub fn update(&mut self, value: bool) {
        self.total_values_observed += 1;
        if value {
            self.total_true_values_observed += 1;
        }
    }

    pub fn merge(&mut self, other: &BoolPattern) {
        self.total_values_observed += other.total_values_observed;
        self.total_true_values_observed += other.total_true_values_observed;
    }

    pub fn into_json<T: rand::Rng>(&self, rng: &mut T) -> serde_json::Value {
        serde_json::Value::Bool(rng.gen_ratio(self.total_true_values_observed, self.total_values_observed))
    }
}
