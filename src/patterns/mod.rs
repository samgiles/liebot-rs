mod array_pattern;
mod bool_pattern;
mod object;
mod range;
mod string_pattern;

pub use array_pattern::ArrayPattern;
pub use bool_pattern::BoolPattern;
pub use object::ObjectPattern;
pub use range::Range;
pub use string_pattern::StringPattern;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Null,
    Bool(BoolPattern),
    Number(Range<f64>),
    Object(ObjectPattern),
    String(StringPattern),
    Array(ArrayPattern),
}

impl Pattern {
    pub fn merge(&mut self, other_pattern: &Pattern) {
        match (&mut *self, &*other_pattern) {
            (Pattern::Null, &Pattern::Bool(ref other)) => {
                *self = Pattern::Bool(other.clone());
            },
            (Pattern::Null, &Pattern::Number(ref other)) => {
                *self = Pattern::Number(other.clone());
            },
            (Pattern::Null, &Pattern::Object(ref other)) => {
                *self = Pattern::Object(other.clone());
            },
            (Pattern::Null, &Pattern::String(ref other)) => {
                *self = Pattern::String(other.clone());
            },
            (Pattern::Null, &Pattern::Array(ref other)) => {
                *self = Pattern::Array(other.clone());
            },
            (Pattern::Bool(ref mut this), &Pattern::Bool(ref other)) => {
                this.merge(other);
            },
            (Pattern::Number(ref mut this), &Pattern::Number(ref other)) => {
                this.merge(other);
            },
            (Pattern::Object(ref mut this), &Pattern::Object(ref other)) => {
                this.merge(other);
            },
            (Pattern::String(ref mut this), &Pattern::String(ref other)) => {
                this.merge(other);
            },
            (Pattern::Array(ref mut this), &Pattern::Array(ref other)) => {
                this.merge(other);
            },
            (_, _) => {
                // cannot merge different types
            }
        }
    }

    pub fn into_json<T: rand::Rng>(&self, rng: &mut T) -> serde_json::Value {
        match self {
            Pattern::Null => {
                serde_json::Value::Null 
            },
            Pattern::Bool(this) => {
                this.into_json(rng)
            },
            Pattern::Number(this) => {
                this.into_json(rng)
            },
            Pattern::Object(this) => {
                this.into_json(rng)
            },
            Pattern::String(this) => {
                this.into_json(rng)
            },
            Pattern::Array(this) => {
                this.into_json(rng)
            }
        }
    }
}
