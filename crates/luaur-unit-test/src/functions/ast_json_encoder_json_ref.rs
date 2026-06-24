use crate::functions::ast_json_encoder_json::json;
use alloc::string::String;

pub fn json_ref<T>(node: &mut T) -> String {
    json(node as *mut T)
}
