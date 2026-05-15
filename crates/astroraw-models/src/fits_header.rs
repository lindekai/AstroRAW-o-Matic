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

/// Helper trait so push_* methods accept both `"string literal"` and `None`.
pub trait IntoComment {
    fn into_comment(self) -> Option<String>;
}

impl IntoComment for &str {
    fn into_comment(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl IntoComment for String {
    fn into_comment(self) -> Option<String> {
        Some(self)
    }
}

impl IntoComment for Option<String> {
    fn into_comment(self) -> Option<String> {
        self
    }
}

impl IntoComment for Option<&str> {
    fn into_comment(self) -> Option<String> {
        self.map(|s| s.to_string())
    }
}

// Allows passing bare `None` without a type annotation
impl IntoComment for () {
    fn into_comment(self) -> Option<String> {
        None
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

    pub fn push(&mut self, keyword: impl Into<String>, value: FitsValue, comment: impl IntoComment) {
        self.records.push(FitsHeaderRecord {
            keyword: keyword.into(),
            value,
            comment: comment.into_comment(),
        });
    }

    pub fn push_str(&mut self, keyword: impl Into<String>, value: impl Into<String>, comment: impl IntoComment) {
        self.push(keyword, FitsValue::Str(value.into()), comment);
    }

    pub fn push_int(&mut self, keyword: impl Into<String>, value: i64, comment: impl IntoComment) {
        self.push(keyword, FitsValue::Int(value), comment);
    }

    pub fn push_float(&mut self, keyword: impl Into<String>, value: f64, comment: impl IntoComment) {
        self.push(keyword, FitsValue::Float(value), comment);
    }

    pub fn push_bool(&mut self, keyword: impl Into<String>, value: bool, comment: impl IntoComment) {
        self.push(keyword, FitsValue::Bool(value), comment);
    }
}
