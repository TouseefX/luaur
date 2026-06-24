use crate::records::constraint_block::ConstraintBlock;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct ConstraintSnapshot {
    pub stringification: alloc::string::String,
    pub location: Location,
    pub blocks: alloc::vec::Vec<ConstraintBlock>,
}

// A `DenseHashMap<*const Constraint, ConstraintSnapshot>` value-initializes the
// slot on `operator[]` (C++ `target.unsolvedConstraints[c.get()]`), i.e. a
// default-constructed `ConstraintSnapshot{}`.
impl luaur_common::records::dense_hash_table::DenseDefault for ConstraintSnapshot {
    fn dense_default() -> Self {
        Self {
            stringification: alloc::string::String::new(),
            location: Location::default(),
            blocks: alloc::vec::Vec::new(),
        }
    }
}
