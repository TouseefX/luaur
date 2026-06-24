use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::cannot_extend_table::{CannotExtendTable, Context};
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_15(&self, e: &CannotExtendTable) -> String {
        match e.context() {
            Context::Property => {
                "Cannot add property '".to_string()
                    + e.prop()
                    + "' to table '"
                    + &to_string_type_id(e.tableType())
                    + "'"
            }
            Context::Metatable => {
                "Cannot add metatable to table '".to_string()
                    + &to_string_type_id(e.tableType())
                    + "'"
            }
            Context::Indexer => {
                "Cannot add indexer to table '".to_string()
                    + &to_string_type_id(e.tableType())
                    + "'"
            }
        }
    }
}
