use crate::records::path_hash::PathHash;
use crate::type_aliases::component::Component;

impl PathHash {
    pub fn operator_call(&self, component: &Component) -> usize {
        self.operator_component(component)
    }
}
