use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::fresh_index::fresh_index;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::subsumes_scope::subsumes;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generalization_params::GeneralizationParams;
use crate::records::generalization_result::GeneralizationResult;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;

/// C++ `GeneralizationResult<TypePackId> generalizeTypePack(...)`
/// (Generalization.cpp:839-868). Generalize one type pack.
pub fn generalize_type_pack(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    scope: *mut Scope,
    tp: TypePackId,
    params: &GeneralizationParams,
) -> GeneralizationResult {
    let tp = unsafe { follow_type_pack_id(tp) };

    if unsafe { (*tp).owningArena } != arena {
        return not_replaced(tp);
    }

    let ftp = unsafe { get_type_pack_id::<FreeTypePack>(tp) };
    if ftp.is_null() {
        return not_replaced(tp);
    }

    if !subsumes(scope, unsafe { (*ftp).scope }) {
        return not_replaced(tp);
    }

    if params.use_count == 1 {
        // emplaceTypePack<BoundTypePack>(asMutable(tp), builtinTypes->unknownTypePack)
        unsafe {
            (*(tp as *mut TypePackVar)).ty =
                TypePackVariant::Bound((*builtin_types).unknownTypePack);
        }
    } else {
        // emplaceTypePack<GenericTypePack>(asMutable(tp), scope, params.polarity)
        emplace_generic_pack(tp, scope, params.polarity);
        return GeneralizationResult {
            // `GeneralizationResult` is monomorphized to `TypeId` in this port;
            // the pack id is stored via a pointer cast (the driver only checks
            // `result.is_some()` / `was_replaced_by_generic` and forwards the
            // original `freePack`, never dereferencing `result`).
            result: Some(tp as crate::type_aliases::type_id::TypeId),
            was_replaced_by_generic: true,
            resource_limits_exceeded: false,
        };
    }

    not_replaced(tp)
}

#[inline]
fn not_replaced(tp: TypePackId) -> GeneralizationResult {
    GeneralizationResult {
        result: Some(tp as crate::type_aliases::type_id::TypeId),
        was_replaced_by_generic: false,
        resource_limits_exceeded: false,
    }
}

/// C++ `emplaceTypePack<GenericTypePack>(asMutable(tp), scope, polarity)`:
/// matches `GenericTypePack{scope, polarity}`.
fn emplace_generic_pack(
    tp: TypePackId,
    scope: *mut Scope,
    polarity: crate::enums::polarity::Polarity,
) {
    let gtp = GenericTypePack {
        index: fresh_index(),
        level: Default::default(),
        scope,
        name: Default::default(),
        explicitName: false,
        polarity,
    };
    unsafe {
        (*(tp as *mut TypePackVar)).ty = TypePackVariant::Generic(gtp);
    }
}
