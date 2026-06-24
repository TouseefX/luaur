use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use core::ptr;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

#[derive(Debug, Clone)]
pub struct InstanceCollector2 {
    pub base: TypeOnceVisitor,
    pub tys: VecDeque<TypeId>,
    pub tps: VecDeque<TypePackId>,
    pub cyclic_instance: DenseHashSet<TypeId>,
    pub instance_arguments: DenseHashSet<TypeId>,
}

impl InstanceCollector2 {
    pub fn new() -> Self {
        Self {
            base: TypeOnceVisitor::new(String::from("InstanceCollector2"), true),
            tys: VecDeque::new(),
            tps: VecDeque::new(),
            cyclic_instance: DenseHashSet::new(ptr::null()),
            instance_arguments: DenseHashSet::new(ptr::null()),
        }
    }

    pub fn visit_type_function_instance_type(
        &mut self,
        ty: TypeId,
        it: &TypeFunctionInstanceType,
    ) -> bool {
        self.tys.push_front(ty);
        for &t in &it.type_arguments {
            let followed = unsafe { follow_type_id(t) };
            self.instance_arguments.insert(followed);
        }
        true
    }

    // `cycle` lives in its own method node file (methods/instance_collector_2_cycle.rs).

    pub fn visit_extern_type(&mut self, _ty: TypeId, _et: &ExternType) -> bool {
        false
    }

    pub fn visit_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        _itp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        self.tps.push_front(tp);
        true
    }
}
