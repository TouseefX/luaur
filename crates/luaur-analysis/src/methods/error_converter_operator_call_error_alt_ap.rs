use crate::records::error_converter::ErrorConverter;
use crate::records::pack_where_clause_needed::PackWhereClauseNeeded;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_49(&self, e: &PackWhereClauseNeeded) -> String {
        let tp = format!("{:?}", e.tp);
        String::from("Type pack function instance ") + &tp + " depends on generic function parameters but does not appear in the function signature; this construct cannot be type-checked at this time"
    }
}
