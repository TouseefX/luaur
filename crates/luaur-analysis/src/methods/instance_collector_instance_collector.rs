use crate::records::instance_collector::InstanceCollector;
use crate::records::type_once_visitor::TypeOnceVisitor;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

impl InstanceCollector {
    pub fn instance_collector(&mut self) {
        self.base = TypeOnceVisitor::new("InstanceCollector".to_string(), true);
        self.recorded_tys = DenseHashSet::new(crate::type_aliases::type_id::TypeId::default());
        self.tys = VecDeque::new();
        self.recorded_tps =
            DenseHashSet::new(crate::type_aliases::type_pack_id::TypePackId::default());
        self.tps = VecDeque::new();
        self.should_guess =
            crate::type_aliases::type_or_type_pack_id_set::TypeOrTypePackIdSet::default();
        self.type_function_instance_stack = Vec::new();
        self.cyclic_instance = Vec::new();
    }
}
