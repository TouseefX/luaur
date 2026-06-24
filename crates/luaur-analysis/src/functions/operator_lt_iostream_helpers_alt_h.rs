use crate::enums::table_state::TableState;

#[allow(non_snake_case)]
pub fn operator_lt(stream: &mut dyn core::fmt::Write, tv: &TableState) -> core::fmt::Result {
    write!(stream, "{}", *tv as i32)
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_lt as operator_lt_ostream_table_state;
