use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use super::Pattern;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectPattern {
    members: BTreeMap<String, Pattern>,
}

impl ObjectPattern {
    pub fn new(members: BTreeMap<String, Pattern>) -> Self {
        Self { members, }
    }

    pub fn merge(&mut self, other: &ObjectPattern) {

        for (key, value) in other.members.iter() {
            if let Some(this_pattern) = self.members.get_mut(key) {
                this_pattern.merge(value);
            }
        }
    }

    pub fn into_json<T: rand::Rng>(&self, rng: &mut T) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (key, value) in self.members.iter() {
            map.insert(key.into(), value.into_json(rng));
        }

        serde_json::Value::Object(map)
    }
}
