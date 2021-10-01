#![feature(map_first_last)]
mod patterns;
const TEST_API: &str = "https://random-data-api.com/api/device/random_device?size=30";
fn main() {
    println!("Requesting....");
    let response = reqwest::blocking::get(TEST_API).unwrap()
        .json::<serde_json::Value>().unwrap();
    println!("Recieved response...");

    let pattern = into_pattern(&response);

    // todo: don't serialize the string oracle data only the final patterns
//    println!("{}", serde_json::to_string_pretty(&pattern).unwrap());

    let mut rng = rand::thread_rng();
    let fake_response = pattern.into_json(&mut rng);
    println!("{}", serde_json::to_string_pretty(&fake_response).unwrap());
}

use serde_json::Value;
use std::collections::BTreeMap;
use patterns::*;

pub fn into_pattern(json_val: &Value) -> patterns::Pattern {
    match json_val {
       Value::Null => {
           Pattern::Null
       },
       Value::Bool(value) => {
           Pattern::Bool(BoolPattern::new(*value))
       },
       Value::Number(number) => {
           // todo: unwrap and check type.
           let mut range = Range::<f64>::new();
           range.update(number.as_f64().unwrap());
           Pattern::Number(range)
       },
       Value::Object(obj) => {
           let mut obj_pattern = BTreeMap::new();
           for (key, val) in obj.iter() {
               obj_pattern.insert(key.to_string(), into_pattern(val));
           }
           Pattern::Object(ObjectPattern::new(obj_pattern))
       },
       Value::String(string) => {
           Pattern::String(StringPattern::new(string.to_string()))
       },
       Value::Array(values) => {
           let mut size_range = Range::<u64>::new();
           size_range.update(values.len() as u64);
           let mut iterator = values.iter();

           if let Some(pattern) = iterator.next() {
               let mut member_pattern = into_pattern(pattern);

               for val in iterator {
                   member_pattern.merge(&into_pattern(val));
               }

               Pattern::Array(ArrayPattern::new(size_range,  member_pattern))
           } else {
               Pattern::Array(ArrayPattern::new(size_range,  Pattern::Null))
           }

       }
    }
}
