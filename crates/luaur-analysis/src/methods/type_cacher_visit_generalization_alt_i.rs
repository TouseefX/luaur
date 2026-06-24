use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::function_type::FunctionType;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use core::ffi::c_void;

impl TypeCacher {
    /// C++ `bool TypeCacher::visit(TypeId ty, const FunctionType& ft)`
    /// (Generalization.cpp:337-388).
    pub fn visit_type_id_function_type(&mut self, ty: TypeId, ft: &FunctionType) -> bool {
        if self.is_cached(ty) || self.is_uncacheable_type_id(ty) {
            return false;
        }

        cacher_traverse_type_pack_id(self, ft.arg_types);
        cacher_traverse_type_pack_id(self, ft.ret_types);
        for &gen in &ft.generics {
            cacher_traverse_type_id(self, gen);
        }

        let mut uncacheable = false;

        if self.is_uncacheable_type_pack_id(ft.arg_types) {
            uncacheable = true;
        } else if self.is_uncacheable_type_pack_id(ft.ret_types) {
            uncacheable = true;
        }

        // for (TypeId argTy : ft.argTypes) — iterate the flattened arg pack.
        for arg_ty in flatten_type_pack(ft.arg_types) {
            if self.is_uncacheable_type_id(arg_ty) {
                uncacheable = true;
                break;
            }
        }

        for ret_ty in flatten_type_pack(ft.ret_types) {
            if self.is_uncacheable_type_id(ret_ty) {
                uncacheable = true;
                break;
            }
        }

        for &g in &ft.generics {
            if self.is_uncacheable_type_id(g) {
                uncacheable = true;
                break;
            }
        }

        if uncacheable {
            self.mark_uncacheable_type_id(ty);
        } else {
            self.cache(ty);
        }

        false
    }
}

/// C++ range-for over a `TypePackId` (`for (TypeId x : pack)`): walks the head
/// chain following `Bound`/`TypePack` tails, yielding each head element.
fn flatten_type_pack(tp: TypePackId) -> alloc::vec::Vec<TypeId> {
    let mut out = alloc::vec::Vec::new();
    let mut cur = unsafe { follow_type_pack_id(tp) };
    loop {
        let pack = unsafe { get_type_pack_id::<TypePack>(cur) };
        if pack.is_null() {
            break;
        }
        let pack = unsafe { &*pack };
        for &h in &pack.head {
            out.push(h);
        }
        match pack.tail {
            Some(tail) => cur = unsafe { follow_type_pack_id(tail) },
            None => break,
        }
    }
    out
}

/// C++ `TypeOnceVisitor::traverse(TypeId)` for the `TypeCacher`. The cacher's
/// overrides live as inherent methods here; this routes a followed type to the
/// correct typed `visit`. (`follow` already skips `Bound` because the visitor
/// is constructed with `skipBoundTypes = true`.)
pub(crate) fn cacher_traverse_type_id(this: &mut TypeCacher, ty: TypeId) {
    let ty = unsafe { follow_type_id(ty) };
    let seen_key = ty as *mut c_void;
    if this.base.base.seen.contains(&seen_key) {
        return;
    }
    this.base.base.seen.insert(seen_key);

    unsafe {
        if let Some(v) = get_type_id::<FreeType>(ty).as_ref() {
            this.visit_type_id_free_type(ty, v);
        } else if let Some(v) = get_type_id::<GenericType>(ty).as_ref() {
            this.visit_type_id_generic_type(ty, v);
        } else if let Some(v) = get_type_id::<ErrorType>(ty).as_ref() {
            this.visit_type_id_error_type(ty, v);
        } else if let Some(v) = get_type_id::<PrimitiveType>(ty).as_ref() {
            this.visit_type_id_primitive_type(ty, v);
        } else if let Some(v) = get_type_id::<SingletonType>(ty).as_ref() {
            this.visit_type_id_singleton_type(ty, v);
        } else if let Some(v) = get_type_id::<BlockedType>(ty).as_ref() {
            this.visit_type_id_blocked_type(ty, v);
        } else if let Some(v) = get_type_id::<PendingExpansionType>(ty).as_ref() {
            this.visit_type_id_pending_expansion_type(ty, v);
        } else if let Some(v) = get_type_id::<FunctionType>(ty).as_ref() {
            this.visit_type_id_function_type(ty, v);
        } else if let Some(v) = get_type_id::<TableType>(ty).as_ref() {
            this.visit_type_id_table_type(ty, v);
        } else if let Some(v) = get_type_id::<MetatableType>(ty).as_ref() {
            this.visit_type_id_metatable_type(ty, v);
        } else if let Some(v) = get_type_id::<ExternType>(ty).as_ref() {
            this.visit_type_id_extern_type(ty, v);
        } else if let Some(v) = get_type_id::<AnyType>(ty).as_ref() {
            this.visit_type_id_any_type(ty, v);
        } else if let Some(v) = get_type_id::<NoRefineType>(ty).as_ref() {
            this.visit_type_id_no_refine_type(ty, v);
        } else if let Some(v) = get_type_id::<UnionType>(ty).as_ref() {
            this.visit_type_id_union_type(ty, v);
        } else if let Some(v) = get_type_id::<IntersectionType>(ty).as_ref() {
            this.visit_type_id_intersection_type(ty, v);
        } else if let Some(v) = get_type_id::<UnknownType>(ty).as_ref() {
            this.visit_type_id_unknown_type(ty, v);
        } else if let Some(v) = get_type_id::<NeverType>(ty).as_ref() {
            this.visit_type_id_never_type(ty, v);
        } else if let Some(v) = get_type_id::<NegationType>(ty).as_ref() {
            this.visit_type_id_negation_type(ty, v);
        } else if let Some(v) = get_type_id::<TypeFunctionInstanceType>(ty).as_ref() {
            this.visit_type_id_type_function_instance_type(ty, v);
        }
        // Lazy / unhandled variants: the cacher has no override and never
        // legitimately reaches them here.
    }
}

/// C++ `TypeOnceVisitor::traverse(TypePackId)` for the `TypeCacher`.
pub(crate) fn cacher_traverse_type_pack_id(this: &mut TypeCacher, tp: TypePackId) {
    let tp = unsafe { follow_type_pack_id(tp) };
    let seen_key = tp as *mut c_void;
    if this.base.base.seen.contains(&seen_key) {
        return;
    }
    this.base.base.seen.insert(seen_key);

    unsafe {
        if let Some(v) = get_type_pack_id::<FreeTypePack>(tp).as_ref() {
            this.visit_type_pack_id_free_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<GenericTypePack>(tp).as_ref() {
            this.visit_type_pack_id_generic_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<ErrorTypePack>(tp).as_ref() {
            this.visit_type_pack_id_error_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<VariadicTypePack>(tp).as_ref() {
            this.visit_type_pack_id_variadic_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<BlockedTypePack>(tp).as_ref() {
            this.visit_type_pack_id_blocked_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<TypeFunctionInstanceTypePack>(tp).as_ref() {
            this.visit_type_pack_id_type_function_instance_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<BoundTypePack>(tp).as_ref() {
            this.visit_type_pack_id_bound_type_pack(tp, v);
        } else if let Some(v) = get_type_pack_id::<TypePack>(tp).as_ref() {
            this.visit_type_pack_id_type_pack(tp, v);
        }
    }
}
