use crate::functions::are_equivalent::are_equivalent;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::internal_type_function_finder::InternalTypeFunctionFinder;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl InternalTypeFunctionFinder {
    pub fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        let mut has_generic = false;

        for p in &tfitp.typeArguments {
            unsafe {
                if !get_type_id::<GenericType>(follow_type_id(*p)).is_null() {
                    has_generic = true;
                    break;
                }
            }
        }

        if !has_generic {
            for p in &tfitp.packArguments {
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
            for mentioned in self.mentioned_function_packs.iter() {
                let mentioned_tfitp = unsafe {
                    crate::functions::get_type_pack::get_type_pack_id::<TypeFunctionInstanceTypePack>(
                        *mentioned,
                    )
                };
                unsafe {
                    LUAU_ASSERT!(!mentioned_tfitp.is_null());
                    if are_equivalent(tfitp, &*mentioned_tfitp) {
                        return true;
                    }
                }
            }
            self.internal_pack_functions.insert(tp);
        }

        true
    }
}
