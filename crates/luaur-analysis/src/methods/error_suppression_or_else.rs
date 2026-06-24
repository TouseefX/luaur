use crate::enums::value::Value;
use crate::records::error_suppression::ErrorSuppression;

impl ErrorSuppression {
    pub fn error_suppression_or_else(&self, other: &ErrorSuppression) -> ErrorSuppression {
        match self.error_suppression_value() {
            Value::DoNotSuppress => other.clone(),
            _ => self.clone(),
        }
    }
}
