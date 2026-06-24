use crate::records::to_string_span::ToStringSpan;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct ToStringResult {
    pub name: String,
    /// Records which TypeId produced each substring of the output. Only recorded for named types
    pub type_spans: Vec<ToStringSpan>,
    pub invalid: bool,
    pub error: bool,
    pub cycle: bool,
    pub truncated: bool,
}

impl Default for ToStringResult {
    fn default() -> Self {
        Self {
            name: String::new(),
            type_spans: Vec::new(),
            invalid: false,
            error: false,
            cycle: false,
            truncated: false,
        }
    }
}

#[allow(non_snake_case)]
impl ToStringResult {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typeSpans(&self) -> &[ToStringSpan] {
        &self.type_spans
    }

    pub fn invalid(&self) -> bool {
        self.invalid
    }

    pub fn error(&self) -> bool {
        self.error
    }

    pub fn cycle(&self) -> bool {
        self.cycle
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }
}
