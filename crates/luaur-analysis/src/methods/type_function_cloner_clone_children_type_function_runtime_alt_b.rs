use crate::functions::get_mutable_type_function_runtime_alt_f::get_mutable_type_function_type_pack_id;
use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionCloner {
    pub fn clone_children_type_function_type_pack_id_type_function_type_pack_id(
        &mut self,
        tp: TypeFunctionTypePackId,
        tftp: TypeFunctionTypePackId,
    ) {
        unsafe {
            let tPack1: *mut TypeFunctionTypePack =
                get_mutable_type_function_type_pack_id::<TypeFunctionTypePack>(tp);
            let tPack2: *mut TypeFunctionTypePack =
                get_mutable_type_function_type_pack_id::<TypeFunctionTypePack>(tftp);

            if !tPack1.is_null() && !tPack2.is_null() {
                self.clone_children_type_function_type_pack_type_function_type_pack(tPack1, tPack2);
            } else {
                let vPack1: *mut TypeFunctionVariadicTypePack =
                    get_mutable_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp);
                let vPack2: *mut TypeFunctionVariadicTypePack =
                    get_mutable_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tftp);

                if !vPack1.is_null() && !vPack2.is_null() {
                    self.clone_children_type_function_variadic_type_pack_type_function_variadic_type_pack(vPack1, vPack2);
                } else {
                    let gPack1: *mut TypeFunctionGenericTypePack =
                        get_mutable_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
                    let gPack2: *mut TypeFunctionGenericTypePack =
                        get_mutable_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tftp);

                    if !gPack1.is_null() && !gPack2.is_null() {
                        self.clone_children_type_function_generic_type_pack_type_function_generic_type_pack(gPack1, gPack2);
                    } else {
                        LUAU_ASSERT!(false);
                    }
                }
            }
        }
    }
}
