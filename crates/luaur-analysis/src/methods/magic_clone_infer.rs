use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::clone_state::CloneState;
use crate::records::count_mismatch::CountMismatch;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::table_type::TableType;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use alloc::vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn magic_clone_infer(context: &MagicFunctionCallContext) -> bool {
    let solver = unsafe { context.solver.as_ref() };
    let arena = unsafe { &mut *solver.arena };
    let call_site = unsafe { context.call_site.as_ref() };

    let (param_types, _param_tail) = flatten_type_pack_id(context.arguments);
    if param_types.is_empty() || call_site.args.size == 0 {
        unsafe {
            (*context.solver.as_ptr()).report_error_type_error_data_location(
                TypeErrorData::CountMismatch(CountMismatch {
                    expected: 1,
                    actual: 0,
                    ..Default::default()
                }),
                &call_site.arg_location,
            );
        }
        return false;
    }

    let input_type = unsafe { follow_type_id(param_types[0]) };

    if unsafe { get_type_id::<TableType>(input_type) }.is_null() {
        return false;
    }

    let mut clone_state = CloneState {
        builtin_types: solver.builtin_types,
        seen_types: DenseHashMap::new(core::ptr::null()),
        seen_type_packs: DenseHashMap::new(core::ptr::null()),
    };
    let result_type = shallow_clone(
        input_type,
        arena,
        &mut clone_state,
        /* ignorePersistent */ true,
    );

    let constraint_scope = unsafe { (*context.constraint.as_ptr()).scope };

    let table_type = unsafe { get_mutable_type_id::<TableType>(result_type) };
    if !table_type.is_null() {
        unsafe {
            (*table_type).scope = constraint_scope;
        }
    }

    track_interior_free_type(constraint_scope, result_type);

    let cloned_type_pack = arena.add_type_pack_t(TypePack {
        head: vec![result_type],
        tail: None,
    });
    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
        (*result_mut).ty = TypePackVariant::Bound(cloned_type_pack);
    }

    true
}
