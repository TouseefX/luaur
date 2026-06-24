use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_known::is_known;
use crate::functions::is_positive::is_positive;
use crate::functions::remove_type::remove_type;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::generalization_params::GeneralizationParams;
use crate::records::generalization_result::GeneralizationResult;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::r#type::Type;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `GeneralizationResult<TypeId> generalizeType(...)`
/// (Generalization.cpp:730-837). Replace a single free type by its bounds
/// according to the polarity provided.
pub fn generalize_type(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    scope: *mut Scope,
    free_ty: TypeId,
    params: &GeneralizationParams,
) -> GeneralizationResult {
    let free_ty = unsafe { follow_type_id(free_ty) };

    let ft = unsafe { get_mutable_type_id::<FreeType>(free_ty) };
    LUAU_ASSERT!(!ft.is_null());

    LUAU_ASSERT!(is_known(params.polarity));

    let has_lower_bound =
        unsafe { get_type_id::<NeverType>(follow_type_id((*ft).lower_bound)) }.is_null();
    let has_upper_bound =
        unsafe { get_type_id::<UnknownType>(follow_type_id((*ft).upper_bound)) }.is_null();

    let is_within_function = !params.found_outside_functions;

    if !has_lower_bound && !has_upper_bound {
        if !is_within_function {
            emplace_bound(free_ty, unsafe { (*builtin_types).unknownType });
        } else {
            emplace_generic(free_ty, scope, params.polarity);
            return result_generic(free_ty);
        }
    }
    // It is possible that this free type has other free types in its upper or
    // lower bounds. If so we must replace those references with never (lower)
    // or unknown (upper) to avoid tautological bounds like a <: a <: unknown.
    else if is_positive(params.polarity) && !has_upper_bound {
        let lb = unsafe { follow_type_id((*ft).lower_bound) };
        let lower_free = unsafe { get_mutable_type_id::<FreeType>(lb) };
        if !lower_free.is_null() && unsafe { (*lower_free).upper_bound } == free_ty {
            // Generalizing 'a in:  LO <: 'b <: 'a <: UP
            // ... we can hold onto the bound UP and forward it to 'b.
            let upper_bound = unsafe { follow_type_id((*ft).upper_bound) };
            remove_type(arena, builtin_types, upper_bound, free_ty);
            unsafe { (*lower_free).upper_bound = follow_type_id(upper_bound) };
        } else {
            remove_type(arena, builtin_types, lb, free_ty);
        }

        if unsafe { follow_type_id(lb) } != free_ty {
            emplace_bound(free_ty, lb);
        } else if !is_within_function {
            emplace_bound(free_ty, unsafe { (*builtin_types).unknownType });
        } else {
            // if the lower bound is the type in question (eg 'a <: 'a), we
            // don't actually have a lower bound.
            emplace_generic(free_ty, scope, params.polarity);
            return result_generic(free_ty);
        }
    } else {
        let ub = unsafe { follow_type_id((*ft).upper_bound) };
        let upper_free = unsafe { get_mutable_type_id::<FreeType>(ub) };
        if !upper_free.is_null() && unsafe { (*upper_free).lower_bound } == free_ty {
            // Generalizing 'a in:  LO <: 'a <: 'b <: UP
            // ... we can hold onto the bound LO and forward it to 'b.
            let lower_bound = unsafe { follow_type_id((*ft).lower_bound) };
            remove_type(arena, builtin_types, lower_bound, free_ty);
            unsafe { (*upper_free).lower_bound = follow_type_id(lower_bound) };
        } else {
            remove_type(arena, builtin_types, ub, free_ty);
        }

        if unsafe { follow_type_id(ub) } != free_ty {
            emplace_bound(free_ty, ub);
        } else if !is_within_function || params.use_count == 1 {
            // For a free type  A <: 'b < C  we approximately generalize to the
            // intersection of its bounds, clipping the free type from the upper
            // and lower bounds, then cleaning the resulting intersection.
            let lower_bound = unsafe { (*ft).lower_bound };
            remove_type(arena, builtin_types, lower_bound, free_ty);
            let cleaned_ty = unsafe {
                (*arena).add_type(IntersectionType {
                    parts: alloc::vec![(*ft).lower_bound, ub],
                })
            };
            remove_type(arena, builtin_types, cleaned_ty, free_ty);
            emplace_bound(free_ty, cleaned_ty);
        } else {
            // if the upper bound is the type in question, we don't actually
            // have an upper bound.
            emplace_generic(free_ty, scope, params.polarity);
            return result_generic(free_ty);
        }
    }

    GeneralizationResult {
        result: Some(free_ty),
        was_replaced_by_generic: false,
        resource_limits_exceeded: false,
    }
}

/// C++ `emplaceType<BoundType>(asMutable(ty), boundTo)`.
fn emplace_bound(ty: TypeId, bound_to: TypeId) {
    unsafe {
        (*(ty as *mut Type)).ty = TypeVariant::Bound(bound_to);
    }
}

/// C++ `emplaceType<GenericType>(asMutable(ty), scope, polarity)`.
fn emplace_generic(ty: TypeId, scope: *mut Scope, polarity: crate::enums::polarity::Polarity) {
    unsafe {
        (*(ty as *mut Type)).ty =
            TypeVariant::Generic(GenericType::generic_type_scope_polarity(scope, polarity));
    }
}

#[inline]
fn result_generic(free_ty: TypeId) -> GeneralizationResult {
    GeneralizationResult {
        result: Some(free_ty),
        was_replaced_by_generic: true,
        resource_limits_exceeded: false,
    }
}
