use crate::functions::are_equivalent::are_equivalent;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::internal_type_function_finder::InternalTypeFunctionFinder;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl InternalTypeFunctionFinder {
    pub fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        tfit: &TypeFunctionInstanceType,
    ) -> bool {
        let mut has_generic = false;

        for p in &tfit.type_arguments {
            unsafe {
                if !get_type_id::<GenericType>(follow_type_id(*p)).is_null() {
                    has_generic = true;
                    break;
                }
            }
        }

        if !has_generic {
            for p in &tfit.pack_arguments {
                unsafe {
                    if !crate::functions::get_type_pack::get_type_pack_id::<GenericTypePack>(
                        crate::functions::follow_type_pack::follow_type_pack_id(*p),
                    )
                    .is_null()
                    {
                        has_generic = true;
                        break;
                    }
                }
            }
        }

        if has_generic {
            for mentioned in self.mentioned_functions.iter() {
                let mentioned_tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(*mentioned) };
                unsafe {
                    LUAU_ASSERT!(!mentioned_tfit.is_null());
                    if are_equivalent(tfit, &*mentioned_tfit) {
                        return true;
                    }
                }
            }
            self.internal_functions.insert(ty);
        }

        true
    }
}
