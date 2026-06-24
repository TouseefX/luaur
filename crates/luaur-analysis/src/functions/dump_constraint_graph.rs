use crate::functions::to_string_constraint_graph::to_string_constraint_vertex;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use alloc::string::String;

/// C++ `std::string dump(ConstraintVertex vertex)`.
pub fn dump(vertex: ConstraintVertex) -> String {
    let out = unsafe { to_string_constraint_vertex(vertex) };
    std::print!("{}\n", out);
    out
}
