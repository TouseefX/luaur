//! `std::pair<std::vector<TypeId>, std::vector<TypePackId>> saturateArguments(...)`
//! (`Analysis/src/ConstraintSolver.cpp:113-255`, hand-ported faithfully).

use alloc::vec::Vec;

use crate::functions::finite::finite;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::size_type_pack::size;
use crate::records::apply_type_function::ApplyTypeFunction;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn saturate_arguments(
    arena: &mut TypeArena,
    builtin_types: &mut BuiltinTypes,
    fn_def: &TypeFun,
    raw_type_arguments: &Vec<TypeId>,
    raw_pack_arguments: &Vec<TypePackId>,
) -> (Vec<TypeId>, Vec<TypePackId>) {
    let mut saturated_type_arguments: Vec<TypeId> = Vec::new();
    let mut extra_types: Vec<TypeId> = Vec::new();
    let mut saturated_pack_arguments: Vec<TypePackId> = Vec::new();

    for i in 0..raw_type_arguments.len() {
        let ty = raw_type_arguments[i];

        if i < fn_def.type_params().len() {
            saturated_type_arguments.push(ty);
        } else {
            extra_types.push(ty);
        }
    }

    // If we collected extra types, put them in a type pack now. This case is
    // mutually exclusive with the type pack -> type conversion we do below:
    // extraTypes will only have elements in it if we have more types than we
    // have parameter slots for them to go into.
    if !extra_types.is_empty() && !fn_def.type_pack_params().is_empty() {
        saturated_pack_arguments.push(arena.add_type_pack_initializer_list_type_id(&extra_types));
    }

    for i in 0..raw_pack_arguments.len() {
        let tp = raw_pack_arguments[i];

        // If we are short on regular type saturatedTypeArguments and we have a single
        // element type pack, we can decompose that to the type it contains and
        // use that as a type parameter.
        if saturated_type_arguments.len() < fn_def.type_params().len()
            && size(tp, core::ptr::null_mut()) == 1
            && finite(tp, core::ptr::null_mut())
            && first(tp, false).is_some()
            && saturated_pack_arguments.is_empty()
        {
            saturated_type_arguments.push(first(tp, false).unwrap());
        } else if saturated_pack_arguments.len() < fn_def.type_pack_params().len() {
            saturated_pack_arguments.push(tp);
        }
    }

    let types_provided = saturated_type_arguments.len();
    let types_required = fn_def.type_params().len();

    let packs_provided = saturated_pack_arguments.len();
    let packs_required = fn_def.type_pack_params().len();

    // Extra types should be accumulated in extraTypes, not saturatedTypeArguments. Extra
    // packs will be accumulated in saturatedPackArguments, so we don't have an
    // assertion for that.
    luaur_common::macros::luau_assert::LUAU_ASSERT!(types_provided <= types_required);

    // If we didn't provide enough types, but we did provide a type pack, we
    // don't want to use defaults. The rationale for this is that if the user
    // provides a pack but doesn't provide enough types, we want to report an
    // error, rather than simply using the default saturatedTypeArguments, if they exist. If
    // they did provide enough types, but not enough packs, we of course want to
    // use the default packs.
    let needs_defaults = (types_provided < types_required && packs_provided == 0)
        || (types_provided == types_required && packs_provided < packs_required);

    if needs_defaults {
        // Default types can reference earlier types. It's legal to write
        // something like
        // type T<A, B = A> = (A, B) -> number
        // and we need to respect that. We use an ApplyTypeFunction for this.
        // C++ `ApplyTypeFunction atf{arena};`
        let mut atf = ApplyTypeFunction {
            base: Substitution::substitution_new(TxnLog::empty(), arena as *mut TypeArena),
            encountered_forwarded_type: false,
            type_arguments: DenseHashMap::new(core::ptr::null()),
            type_pack_arguments: DenseHashMap::new(core::ptr::null()),
        };

        for i in 0..types_provided {
            *atf.type_arguments.get_or_insert(fn_def.type_params()[i].ty) =
                saturated_type_arguments[i];
        }

        for i in types_provided..types_required {
            let default_ty: TypeId = match fn_def.type_params()[i].defaultValue {
                Some(d) => d,
                // We will fill this in with the error type later.
                None => break,
            };

            let instantiated_default = atf
                .substitute_type_id(default_ty)
                .unwrap_or(builtin_types.errorType);
            *atf.type_arguments.get_or_insert(fn_def.type_params()[i].ty) = instantiated_default;
            saturated_type_arguments.push(instantiated_default);
        }

        for i in 0..packs_provided {
            *atf.type_pack_arguments
                .get_or_insert(fn_def.type_pack_params()[i].tp) = saturated_pack_arguments[i];
        }

        for i in packs_provided..packs_required {
            let default_tp: TypePackId = match fn_def.type_pack_params()[i].defaultValue {
                Some(d) => d,
                // We will fill this in with the error type pack later.
                None => break,
            };

            let instantiated_default = atf
                .substitute_type_pack_id(default_tp)
                .unwrap_or(builtin_types.errorTypePack);
            *atf.type_pack_arguments
                .get_or_insert(fn_def.type_pack_params()[i].tp) = instantiated_default;
            saturated_pack_arguments.push(instantiated_default);
        }
    }

    // If we didn't create an extra type pack from overflowing parameter packs,
    // and we're still missing a type pack, plug in an empty type pack as the
    // value of the empty packs.
    if extra_types.is_empty()
        && saturated_pack_arguments.len() + 1 == fn_def.type_pack_params().len()
    {
        saturated_pack_arguments.push(arena.add_type_pack_initializer_list_type_id(&[]));
    }

    // We need to have _something_ when we substitute the generic saturatedTypeArguments,
    // even if they're missing, so we use the error type as a filler.
    for _ in saturated_type_arguments.len()..types_required {
        saturated_type_arguments.push(builtin_types.errorType);
    }

    for _ in saturated_pack_arguments.len()..packs_required {
        saturated_pack_arguments.push(builtin_types.errorTypePack);
    }

    for arg in saturated_type_arguments.iter_mut() {
        *arg = unsafe { follow_type_id(*arg) };
    }

    for pack in saturated_pack_arguments.iter_mut() {
        *pack = unsafe { follow_type_pack_id(*pack) };
    }

    // At this point, these two conditions should be true. If they aren't we
    // will run into access violations.
    luaur_common::macros::luau_assert::LUAU_ASSERT!(
        saturated_type_arguments.len() == fn_def.type_params().len()
    );
    luaur_common::macros::luau_assert::LUAU_ASSERT!(
        saturated_pack_arguments.len() == fn_def.type_pack_params().len()
    );

    (saturated_type_arguments, saturated_pack_arguments)
}
