use crate::enums::status::Status;
use crate::records::lint_unreachable_code::LintUnreachableCode;

impl LintUnreachableCode {
    pub fn get_reason(&self, status: Status) -> &str {
        match status {
            Status::Continue => "continue",
            Status::Break => "break",
            Status::Return => "return",
            Status::Error => "error",
            _ => "unknown",
        }
    }
}
