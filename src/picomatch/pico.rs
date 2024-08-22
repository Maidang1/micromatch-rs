use crate::types::StringOrArray;
use std::collections::HashMap;

struct PicoMatch {
    glob: StringOrArray,
    options: HashMap<String, String>,
}

impl PicoMatch {
    pub fn is_match(&self, str: StringOrArray, patterns: StringOrArray) -> bool {
        true
    }
}
