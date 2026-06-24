use crate::records::binding_snapshot::BindingSnapshot;
use crate::records::type_binding_snapshot::TypeBindingSnapshot;
use alloc::string::String;
use alloc::vec::Vec;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ScopeSnapshot {
    pub bindings: HashMap<String, BindingSnapshot>,
    pub type_bindings: HashMap<String, TypeBindingSnapshot>,
    pub type_pack_bindings: HashMap<String, TypeBindingSnapshot>,
    pub children: Vec<ScopeSnapshot>,
}
