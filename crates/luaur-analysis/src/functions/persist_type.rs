use alloc::collections::VecDeque;

use crate::enums::table_state::TableState;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::persist_type_alt_b::persist as persist_pack;
use crate::records::any_type::AnyType;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn persist(ty: TypeId) {
    let mut queue: VecDeque<TypeId> = VecDeque::new();
    queue.push_back(ty);

    while let Some(t) = queue.pop_front() {
        unsafe {
            if (*t).persistent {
                continue;
            }

            (*as_mutable_type_id(t)).persistent = true;

            let btv = get_type_id::<BoundType>(t);
            if !btv.is_null() {
                queue.push_back((*btv).boundTo);
                continue;
            }

            let ftv = get_type_id::<FunctionType>(t);
            if !ftv.is_null() {
                persist_pack((*ftv).arg_types);
                persist_pack((*ftv).ret_types);
                continue;
            }

            let ttv = get_type_id::<TableType>(t);
            if !ttv.is_null() {
                LUAU_ASSERT!(
                    (*ttv).state != TableState::Free && (*ttv).state != TableState::Unsealed
                );

                for (_name, prop) in (*ttv).props.iter() {
                    if let Some(read_ty) = prop.read_ty {
                        queue.push_back(read_ty);
                    }
                    if let Some(write_ty) = prop.write_ty {
                        queue.push_back(write_ty);
                    }
                }

                if let Some(indexer) = &(*ttv).indexer {
                    queue.push_back(indexer.index_type);
                    queue.push_back(indexer.index_result_type);
                }
                continue;
            }

            let etv = get_type_id::<ExternType>(t);
            if !etv.is_null() {
                for (_name, prop) in (*etv).props.iter() {
                    if let Some(read_ty) = prop.read_ty {
                        queue.push_back(read_ty);
                    }
                    if let Some(write_ty) = prop.write_ty {
                        queue.push_back(write_ty);
                    }
                }
                continue;
            }

            let utv = get_type_id::<UnionType>(t);
            if !utv.is_null() {
                for &opt in &(*utv).options {
                    queue.push_back(opt);
                }
                continue;
            }

            let itv = get_type_id::<IntersectionType>(t);
            if !itv.is_null() {
                for &opt in &(*itv).parts {
                    queue.push_back(opt);
                }
                continue;
            }

            let mtv = get_type_id::<MetatableType>(t);
            if !mtv.is_null() {
                queue.push_back((*mtv).table());
                queue.push_back((*mtv).metatable());
                continue;
            }

            if !get_type_id::<GenericType>(t).is_null()
                || !get_type_id::<AnyType>(t).is_null()
                || !get_type_id::<FreeType>(t).is_null()
                || !get_type_id::<SingletonType>(t).is_null()
                || !get_type_id::<PrimitiveType>(t).is_null()
                || !get_type_id::<NegationType>(t).is_null()
            {
                // nothing to enqueue for these alternatives
                continue;
            }

            let tfit = get_type_id::<TypeFunctionInstanceType>(t);
            if !tfit.is_null() {
                for &ty in (*tfit).type_arguments.iter() {
                    queue.push_back(ty);
                }

                for &tp in (*tfit).pack_arguments.iter() {
                    persist_pack(tp);
                }
                continue;
            }

            LUAU_ASSERT!(false /* "TypeId is not supported in a persist call" */);
        }
    }
}
