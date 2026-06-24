use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::apply_mapped_generics::ApplyMappedGenerics;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;

impl ApplyMappedGenerics {
    pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
        let env = unsafe { &*self.env };
        if unsafe { get_type_id::<ExternType>(ty).as_ref().is_some() } {
            return true;
        }
        if let Some(f) = unsafe { get_type_id::<FunctionType>(ty).as_ref() } {
            for &g in &f.generics {
                let g = unsafe { follow_type_id(g) };
                if let Some(bounds) =
                    crate::methods::subtyping_bind_generic::dense_hash_map_find_no_default(
                        &env.mapped_generics,
                        &g,
                    )
                {
                    if !bounds.is_empty() {
                        return true;
                    }
                }
            }
        }
        unsafe { (*ty).persistent }
    }
}
