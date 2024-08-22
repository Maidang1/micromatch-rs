use std::{any::Any, collections::HashSet, iter::Map};
use crate::types::StringOrArray;

pub fn is_empty_string(s: &str) -> bool {
    s.is_empty() || s == "./"
}
pub fn has_braces(s: &str) -> bool {
    s.contains('{') && s.contains('}')
}


struct micromatch {}

impl micromatch {
    pub fn matches(list: Vec<String>, patterns: StringOrArray, options: Map<String, String>) {

      let new_patterns = patterns.to_array();
      let new_list = list.iter().map(|item| item.to_string()).collect::<Vec<String>>();
      let omit:HashSet<String> =  HashSet::new();
      let keep:HashSet<String> =  HashSet::new();
      let items:HashSet<String> =  HashSet::new();
      let mut negatives = 0;
    }
}
