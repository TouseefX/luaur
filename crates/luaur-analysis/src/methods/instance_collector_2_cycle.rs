use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::instance_collector_2::InstanceCollector2;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl InstanceCollector2 {
    pub fn cycle(&mut self, ty: TypeId) {
        let t = unsafe { follow_type_id(ty) };
        let it = unsafe { get_type_id::<TypeFunctionInstanceType>(t) };
        if !it.is_null() {
            self.cyclic_instance.insert(t);
        }
    }
}
