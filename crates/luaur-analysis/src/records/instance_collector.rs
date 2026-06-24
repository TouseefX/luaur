use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_type_pack_id_set::TypeOrTypePackIdSet;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

#[derive(Debug, Clone)]
pub struct InstanceCollector {
    pub base: TypeOnceVisitor,
    pub recorded_tys: DenseHashSet<TypeId>,
    pub tys: VecDeque<TypeId>,
    pub recorded_tps: DenseHashSet<TypePackId>,
    pub tps: VecDeque<TypePackId>,
    pub should_guess: TypeOrTypePackIdSet,
    pub type_function_instance_stack: Vec<*const core::ffi::c_void>,
    pub cyclic_instance: Vec<TypeId>,
}
