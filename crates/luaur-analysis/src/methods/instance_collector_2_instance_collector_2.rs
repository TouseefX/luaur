use crate::records::instance_collector_2::InstanceCollector2;
use crate::records::type_once_visitor::TypeOnceVisitor;
use core::ptr;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

impl InstanceCollector2 {
    pub fn instance_collector_2(&mut self) {
        self.base = TypeOnceVisitor::new("InstanceCollector2".to_string(), true);
        self.tys = VecDeque::new();
        self.tps = VecDeque::new();
        self.cyclic_instance = DenseHashSet::new(ptr::null_mut());
        self.instance_arguments = DenseHashSet::new(ptr::null_mut());
    }
}
