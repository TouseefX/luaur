use crate::records::error_converter::ErrorConverter;
use crate::records::only_tables_can_have_methods::OnlyTablesCanHaveMethods;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_36(&self, e: &OnlyTablesCanHaveMethods) -> String {
        "Cannot add method to non-table type '".to_string() + &format!("{:?}", e.table_type) + "'"
    }
}
