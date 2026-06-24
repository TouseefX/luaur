extern crate alloc;

use crate::functions::to_dot_to_dot::to_dot_type_id_to_dot_options;
use crate::records::to_dot_options::ToDotOptions;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

pub fn to_dot(ty: TypeId) -> String {
    to_dot_type_id_to_dot_options(ty, &ToDotOptions::default())
}
