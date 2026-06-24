use crate::records::error_converter::ErrorConverter;
use crate::records::where_clause_needed::WhereClauseNeeded;
use alloc::string::String;

impl ErrorConverter {
    pub fn operator_call_63(&self, e: &WhereClauseNeeded) -> String {
        let ty = format!("{:?}", e.ty);
        String::from("Type function instance ") + &ty + " depends on generic function parameters but does not appear in the function signature; this construct cannot be type-checked at this time"
    }
}
