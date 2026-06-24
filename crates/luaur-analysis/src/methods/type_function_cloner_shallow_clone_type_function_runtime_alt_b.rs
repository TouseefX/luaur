//! Faithful port of `TypeFunctionCloner::shallowClone(TypeFunctionTypePackId tp)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2715-2734).
use crate::functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id;
use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionCloner {
    pub fn shallow_clone_type_function_type_pack_id(
        &mut self,
        tp: TypeFunctionTypePackId,
    ) -> TypeFunctionTypePackId {
        // if (auto it = find(tp))
        //     return *it;
        if let Some(it) = self.find_type_function_type_pack_id(tp) {
            return it;
        }

        unsafe {
            let runtime = self.type_function_runtime;

            // Create a shallow serialization
            // TypeFunctionTypePackId target = {};
            let mut target: TypeFunctionTypePackId = core::ptr::null();

            if !get_type_function_type_pack_id::<TypeFunctionTypePack>(tp).is_null() {
                target = (*runtime)
                    .type_pack_arena
                    .allocate(TypeFunctionTypePackVar::new(
                        TypeFunctionTypePackVariant::V0(TypeFunctionTypePack {
                            head: Vec::new(),
                            tail: None,
                        }),
                    ));
            } else if !get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp).is_null()
            {
                target = (*runtime)
                    .type_pack_arena
                    .allocate(TypeFunctionTypePackVar::new(
                        TypeFunctionTypePackVariant::V1(TypeFunctionVariadicTypePack {
                            type_id: core::ptr::null(),
                        }),
                    ));
            } else if let Some(g_pack) =
                get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp).as_ref()
            {
                target = (*runtime)
                    .type_pack_arena
                    .allocate(TypeFunctionTypePackVar::new(
                        TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
                            is_named: g_pack.is_named,
                            name: g_pack.name.clone(),
                        }),
                    ));
            } else {
                LUAU_ASSERT!(false /* "Unknown type" */);
            }

            // packs[tp] = target;
            *self.packs.get_or_insert(tp) = target;
            // queue.emplace_back(tp, target);
            self.queue.push((
                crate::type_aliases::type_function_kind::TypeFunctionKind::V1(tp),
                crate::type_aliases::type_function_kind::TypeFunctionKind::V1(target),
            ));
            target
        }
    }
}
