pub enum StringOrArray {
    String(String),
    Array(Vec<String>),
}

impl StringOrArray {
    // 方法 1: 实现为 StringOrArray 的方法
    pub fn to_array(self) -> Vec<String> {
        match self {
            StringOrArray::String(s) => vec![s],
            StringOrArray::Array(arr) => arr,
        }
    }
}
