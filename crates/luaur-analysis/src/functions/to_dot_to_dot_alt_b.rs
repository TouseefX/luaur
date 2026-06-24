extern crate alloc;

use crate::records::state_dot::StateDot;
use crate::records::to_dot_options::ToDotOptions;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;

pub fn to_dot(tp: TypePackId, opts: &ToDotOptions) -> String {
    let mut state = StateDot::new(*opts);

    state.result = String::from("digraph graphname {\n");
    state.visit_child_type_pack_id_i32_c_char(tp, 0, core::ptr::null());
    state.result.push('}');

    state.result
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use to_dot as to_dot_type_pack_id_to_dot_options;
