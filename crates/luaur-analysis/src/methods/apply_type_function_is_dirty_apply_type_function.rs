use crate::functions::get_type_alt_j::get_type_id;
use crate::records::apply_type_function::ApplyTypeFunction;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;

impl ApplyTypeFunction {
    pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
        if self.type_arguments.find(&ty).is_some() {
            true
        } else {
            let ftv = unsafe { get_type_id::<FreeType>(ty) };
            if ftv.is_null() {
                false
            } else {
                unsafe {
                    if (*ftv).forwarded_type_alias {
                        self.encountered_forwarded_type = true;
                    }
                }
                false
            }
        }
    }
}
