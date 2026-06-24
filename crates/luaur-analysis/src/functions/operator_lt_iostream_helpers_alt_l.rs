use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::type_aliases::type_pack_id::TypePackId;
use core::fmt::Write;

#[allow(non_snake_case)]
pub fn operator_lt_ostream_type_pack_id(
    stream: &mut dyn Write,
    tp: TypePackId,
) -> core::fmt::Result {
    if tp.is_null() {
        return write!(stream, "<nullptr>");
    }

    let s = to_string_type_pack_id(tp);
    write!(stream, "{}", s)
}

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn Write, tp: TypePackId) -> core::fmt::Result {
    operator_lt_ostream_type_pack_id(stream, tp)
}

#[allow(unused_imports, non_snake_case)]
pub use operator_lt_ostream_type_pack_id as operator_lt_ostream_type_pack_id_alt;
