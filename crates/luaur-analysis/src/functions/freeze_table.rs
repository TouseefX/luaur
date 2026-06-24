use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::records::clone_state::CloneState;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_mismatch::TypeMismatch;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::FFlag;

pub fn freeze_table(input_type: TypeId, context: &MagicFunctionCallContext) -> Option<TypeId> {
    let solver = unsafe { context.solver.as_ref() };
    let arena = unsafe { &mut *solver.arena };
    let input_type = unsafe { follow_type_id(input_type) };

    let mt = unsafe { get_type_id::<MetatableType>(input_type) };
    if !mt.is_null() {
        let mt_table = unsafe { (*mt).table };
        let frozen_table = freeze_table(mt_table, context)?;

        let result_type = unsafe { &mut *solver.arena }.add_type(MetatableType {
            table: frozen_table,
            metatable: unsafe { (*mt).metatable },
            syntheticName: unsafe { (*mt).syntheticName.clone() },
        });

        return Some(result_type);
    }

    if !unsafe { get_type_id::<TableType>(input_type) }.is_null() {
        // Clone the input type, this will become our final result type after we mutate it.
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
        let table_ty = unsafe { get_mutable_type_id::<TableType>(result_type) };
        // `clone` should not break this.
        luaur_common::macros::luau_assert::LUAU_ASSERT!(!table_ty.is_null());
        unsafe {
            (*table_ty).state = crate::enums::table_state::TableState::Sealed;
        }

        // We'll mutate the table to make every property type read-only.
        let table_ty = unsafe { &mut *table_ty };
        table_ty.props.retain(|_name, prop| !prop.is_write_only());
        for (_name, prop) in table_ty.props.iter_mut() {
            prop.write_ty = None;
        }

        return Some(result_type);
    }

    if !FFlag::LuauTableFreezeCheckIsSubtype.get() {
        let call_site = unsafe { context.call_site.as_ref() };
        let table_type = unsafe { &*solver.builtin_types }.tableType;
        unsafe {
            (*context.solver.as_ptr()).report_error_type_error_data_location(
                TypeErrorData::TypeMismatch(TypeMismatch::from_wanted_given(
                    table_type, input_type,
                )),
                &call_site.arg_location,
            );
        }
    }
    None
}
