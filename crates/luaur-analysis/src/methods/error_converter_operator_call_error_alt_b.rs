use crate::records::error_converter::ErrorConverter;
use crate::records::unknown_symbol::UnknownSymbol;
use alloc::string::String;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ErrorConverter {
    pub fn operator_call_45(&self, e: &UnknownSymbol) -> String {
        match e.context() {
            crate::records::unknown_symbol::Context::Binding => {
                let mut result = String::from("Unknown global '");
                result.push_str(e.name());
                result.push_str("'; consider assigning to it first");
                result
            }
            crate::records::unknown_symbol::Context::Type => {
                let mut result = String::from("Unknown type '");
                result.push_str(e.name());
                result.push('\'');
                result
            }
        }

        // Keep same structure as the C++ version: this branch is unreachable.
        // If a new context variant is added, compilation will fail due to non-exhaustive match.
    }
}
