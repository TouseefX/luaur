use crate::functions::follow_type::follow_type_id;
use crate::records::instance_collector_2::InstanceCollector2;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl InstanceCollector2 {
    pub fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        it: &TypeFunctionInstanceType,
    ) -> bool {
        self.tys.push_front(ty);
        for t in &it.type_arguments {
            let followed = unsafe { follow_type_id(*t) };
            self.instance_arguments.insert(followed);
        }
        true
    }
}
