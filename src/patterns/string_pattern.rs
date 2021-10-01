use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StringPattern {
    inserted_strings: usize,
    max_length_string: usize,
    oracle: BTreeSet<String>,
}

impl StringPattern {
    pub fn new(initial: String) -> Self {
        let mut oracle = BTreeSet::new();
        let length = initial.len();
        oracle.insert(initial);

        Self {
            inserted_strings: 1,
            max_length_string: length,
            oracle,
        }
    }

    pub fn merge(&mut self, other: &StringPattern)  {
        let mut other_oracle = other.oracle.clone();
        self.inserted_strings += other.inserted_strings;
        self.oracle.append(&mut other_oracle);
        self.max_length_string = usize::max(self.max_length_string, other.max_length_string);
    }

    fn maybe_enum_like_field(&self) -> bool {
        self.inserted_strings == self.oracle.len() && self.max_length_string < 20
    }

    pub fn into_json<T: rand::Rng>(&self, rng: &mut T) -> serde_json::Value {
        if self.maybe_enum_like_field() {
            serde_json::Value::String(String::from(self.oracle
                    .iter()
                    .nth(rng.gen_range(0..self.oracle.len()))
                    .unwrap()))
        } else if self.oracle.len() == 1 {
            serde_json::Value::String(String::from(self.oracle.first().unwrap()))
        } else {
            if self.max_length_string > 100 {
                let range = rng.gen_range(0..self.max_length_string); 
                let string: String = std::iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .take(range)
                    .map(char::from)
                    .collect();
                serde_json::Value::String(String::from(string))  
            } else {
                let patterns = self.find_pattern();
                let string = into_string(patterns, rng);
                serde_json::Value::String(string)
            }
        }
    }

    fn find_pattern(&self) -> Vec<StringPatternToken> {
        // this algorithm is... a hack
        let mut pattern_graph = Vec::new();
        // iterate over each seen string and break into a set for each character in order
        for string in self.oracle.iter() {
            for (i, cluster) in string.graphemes(true).enumerate() {
                if pattern_graph.len() < i + 1 {
                    pattern_graph.resize(i + 1, BTreeSet::new());
                }
                pattern_graph[i].insert(cluster);
            }
        }

        let mut patterns = Vec::new();

        // iterate over each cluster and smash them together
        // into something like to find the random portions of a string
        // [Literal("HelloWorld/"), Random(5), Literal("/turtles")]
        // Todo: try to understand the pattern of the random portions? Is it numeric, in a certain
        // range etc.
        for cluster in pattern_graph.iter() {
            match patterns.last_mut() {
                Some(StringPatternToken::Literal(ref mut s)) if cluster.len() == 1 => {
                    s.push_str(cluster.first().unwrap());
                },
                Some(StringPatternToken::Literal(_)) => {
                    patterns.push(StringPatternToken::Random(1));
                },
                Some(StringPatternToken::Random(_)) if cluster.len() == 1 => {
                    patterns.push(StringPatternToken::Literal(cluster.first().unwrap().to_string()));
                },
                Some(StringPatternToken::Random(s)) => {
                    *s += 1;
                },
                Some(_) => {
                },
                None if cluster.len() == 1 => {
                    patterns.push(StringPatternToken::Literal(cluster.first().unwrap().to_string()));
                },
                None => {
                    patterns.push(StringPatternToken::Random(1));
                }
            }
        }

        patterns
    }
}

use rand::{distributions::Alphanumeric};

fn into_string<T: rand::Rng>(pattern: Vec<StringPatternToken>, mut rng: T) -> String {
    let mut builder = String::from("");

    for token in pattern {
        match token {
            StringPatternToken::Literal(s) => {
                builder.push_str(&s);
            },
            StringPatternToken::Random(length) => {
                let string: String = std::iter::repeat(())
                    .map(|()| rng.sample(Alphanumeric))
                    .take(length)
                    .map(char::from)
                    .collect();
                builder.push_str(&string);
            },
            StringPatternToken::Uuid => {
                builder.push_str("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx");
            },
        }
    }

    builder
}

#[derive(Debug)]
enum StringPatternToken {
    Literal(String),
    // todo: constraints
    Random(usize),
    Uuid,
}
