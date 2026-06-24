use crate::records::deprecated_api_used::DeprecatedApiUsed;
use crate::records::error_converter::ErrorConverter;
use alloc::format;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_20(&self, e: &DeprecatedApiUsed) -> String {
        format!(
            "The property .{} is deprecated.  Use .{} instead.",
            e.symbol, e.use_instead
        )
    }
}
