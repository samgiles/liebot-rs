use rand::Rng;
use super::{Pattern, Range};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArrayPattern {
    length: Range<u64>,
    member_pattern: Box<Pattern>,
}

impl ArrayPattern {
    pub fn new(length: Range<u64>, pattern: Pattern) -> Self {
        Self {
            length,
            member_pattern: Box::new(pattern),
        }
    }

    pub fn merge(&mut self, pattern: &ArrayPattern) {
        self.length.merge(&pattern.length);
        self.member_pattern.merge(&*pattern.member_pattern);
    }

    pub fn into_json<T: Rng>(&self, rng: &mut T) -> serde_json::Value {
        let length = self.length.select(rng);
        let mut values = Vec::new();
        for _ in 0..length {
            values.push((&*self.member_pattern).into_json(rng));
        }

        serde_json::Value::Array(values)
    }
}
