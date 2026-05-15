use serde::{Deserialize, Serialize};

/// A single resolved FITS header record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitsHeaderRecord {
    pub keyword: String,
    pub value: FitsValue,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FitsValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

impl std::fmt::Display for FitsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FitsValue::Bool(b) => write!(f, "{}", if *b { "T" } else { "F" }),
            FitsValue::Int(i) => write!(f, "{}", i),
            FitsValue::Float(v) => write!(f, "{:.6}", v),
            FitsValue::Str(s) => write!(f, "'{}'", s),
        }
    }
}

/// The full resolved FITS header for one output file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FitsHeader {
    pub records: Vec<FitsHeaderRecord>,
}

impl FitsHeader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, keyword: impl Into<String>, value: FitsValue, comment: impl Into<Option<String>>) {
        self.records.push(FitsHeaderRecord {
            keyword: keyword.into(),
            value,
            comment: comment.into(),
        });
    }

    pub fn push_str(&mut self, keyword: impl Into<String>, value: impl Into<String>, comment: impl Into<Option<String>>) {
        self.push(keyword, FitsValue::Str(value.into()), comment);
    }

    pub fn push_int(&mut self, keyword: impl Into<String>, value: i64, comment: impl Into<Option<String>>) {
        self.push(keyword, FitsValue::Int(value), comment);
    }

    pub fn push_float(&mut self, keyword: impl Into<String>, value: f64, comment: impl Into<Option<String>>) {
        self.push(keyword, FitsValue::Float(value), comment);
    }

    pub fn push_bool(&mut self, keyword: impl Into<String>, value: bool, comment: impl Into<Option<String>>) {
        self.push(keyword, FitsValue::Bool(value), comment);
    }
}
