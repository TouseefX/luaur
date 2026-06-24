use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::persist_type::persist as persist_type;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn persist(tp: TypePackId) {
    unsafe {
        if (*tp).persistent {
            return;
        }

        (*as_mutable_type_pack(tp)).persistent = true;

        let p = get_type_pack_id::<TypePack>(tp);
        if !p.is_null() {
            for &ty in &(*p).head {
                persist_type(ty);
            }
            if let Some(tail) = (*p).tail {
                persist(tail);
            }
            return;
        }

        let vtp = get_type_pack_id::<VariadicTypePack>(tp);
        if !vtp.is_null() {
            persist_type((*vtp).ty);
            return;
        }

        if !get_type_pack_id::<GenericTypePack>(tp).is_null() {
            return;
        }

        let tfitp = get_type_pack_id::<TypeFunctionInstanceTypePack>(tp);
        if !tfitp.is_null() {
            for &ty in (*tfitp).typeArguments.iter() {
                persist_type(ty);
            }

            for &tp in (*tfitp).packArguments.iter() {
                persist(tp);
            }
            return;
        }

        LUAU_ASSERT!(false /* "TypePackId is not supported in a persist call" */);
    }
}
