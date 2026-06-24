use alloc::string::String;
use core::fmt::Arguments;

pub fn operator_lt_ostream_optional_t<T: core::fmt::Display>(t: Option<T>) -> String {
    match t {
        Some(v) => format!("{}", v),
        None => "none".to_owned(),
    }
}
