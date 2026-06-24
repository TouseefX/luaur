use crate::functions::parse_f_value_helper::parse_f_value_helper;
use crate::type_aliases::f_value_result::FValueResult;
use alloc::string::String;

pub fn parse_f_int(view: &str) -> FValueResult<i32> {
    let (name, value) = parse_f_value_helper(view);
    let value = value.unwrap_or_else(|| {
        panic!("Expected a value associated with {}", name);
    });
    let int_value: i32 = value.parse().expect("Failed to parse integer");
    (name, int_value)
}
