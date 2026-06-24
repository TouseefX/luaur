use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::instance_collector::InstanceCollector;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl InstanceCollector {
    pub fn cycle(&mut self, ty: TypeId) {
        let t = unsafe { follow_type_id(ty) };

        let it = unsafe { get_type_id::<TypeFunctionInstanceType>(t) };
        if !it.is_null() {
            // If we see a type a second time and it's in the type function stack, it's a real cycle
            if self
                .type_function_instance_stack
                .iter()
                .any(|&x| x == t as *const core::ffi::c_void)
            {
                self.cyclic_instance.push(t);
            }
        }
    }
}
