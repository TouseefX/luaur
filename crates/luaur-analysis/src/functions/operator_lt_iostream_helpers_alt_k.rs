use crate::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
use crate::records::to_string_options::ToStringOptions;
use crate::type_aliases::type_id::TypeId;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn Write, ty: TypeId) -> core::fmt::Result {
    if ty.is_null() {
        return stream.write_str("<nullptr>");
    }

    let opts = ToStringOptions::default();
    let s = to_string_type_id_to_string_options_mut(ty, opts);
    write!(stream, "{}", s)
}

#[allow(unused_imports, non_snake_case)]
pub use operator_lt as operator_lt_ostream_type_id;
