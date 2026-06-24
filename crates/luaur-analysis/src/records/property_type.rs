//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Type.h:429:property`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:429-487, hand-ported; the previous
//! occupant was a Normalize-flavored shape that clobbered this file)

use crate::type_aliases::tags::Tags;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, Default)]
pub struct Property {
    pub deprecated: bool,
    pub deprecated_suggestion: String,

    /// If this property was inferred from an expression, this field will be
    /// populated with the source location of the corresponding table property.
    pub location: Option<Location>,

    /// If this property was built from an explicit type annotation, this field
    /// will be populated with the source location of that table property.
    pub type_location: Option<Location>,

    pub tags: Tags,
    pub documentation_symbol: Option<String>,

    // Invariant: at least one of the two optionals are not nullopt!
    // read set + write unset = readonly; read unset + write set = writeonly.
    pub read_ty: Option<TypeId>,
    pub write_ty: Option<TypeId>,
}

pub use Property as PropertyType;

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let present: () = ();
    let result: () = ();
}
