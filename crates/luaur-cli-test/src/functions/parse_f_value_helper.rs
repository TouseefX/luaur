use crate::type_aliases::f_value_result::FValueResult;
use alloc::string::String;

pub fn parse_f_value_helper(view: &str) -> FValueResult<Option<String>> {
    if let Some((name, value)) = view.split_once('=') {
        (name.to_string(), Some(value.to_string()))
    } else {
        (view.to_string(), None)
    }
}
