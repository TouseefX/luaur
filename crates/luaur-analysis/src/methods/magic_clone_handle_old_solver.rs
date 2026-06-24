use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::records::clone_state::CloneState;
use crate::records::count_mismatch::CountMismatch;
use crate::records::intersection_type::IntersectionType;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::sync::Arc;
use alloc::vec;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_common::records::dense_hash_map::DenseHashMap;

// ({+ +}) -> {+ +}
// <T: {}>(T) -> T
pub fn magic_clone_handle_old_solver(
    typechecker: &mut TypeChecker,
    _scope: &Arc<Scope>,
    expr: &AstExprCall,
    with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    let param_pack = with_predicate.r#type;

    let builtin_types = typechecker.builtin_types;
    let module = typechecker.current_module.as_ref()?;
    let arena = unsafe { &mut (*(Arc::as_ptr(module) as *mut Module)).internal_types };

    // in the old solver, nonstrict in particular is really bad about inferring `...any` for things that are definitely present
    // and the only real way for us to deal with this is to just be more permissive here
    let extended = extend_type_pack(arena, builtin_types, param_pack, 1, alloc::vec::Vec::new());
    let param_types = extended.head;
    if param_types.is_empty() || expr.args.size == 0 {
        typechecker.report_error_location_type_error_data(
            &expr.arg_location,
            TypeErrorData::CountMismatch(CountMismatch {
                expected: 1,
                actual: 0,
                ..Default::default()
            }),
        );
        return None;
    }

    let input_type = unsafe { follow_type_id(param_types[0]) };

    let table_ty = unsafe { get_type_id::<TableType>(input_type) };
    let intersection_ty = unsafe { get_type_id::<IntersectionType>(input_type) };
    if table_ty.is_null() && intersection_ty.is_null() {
        return None;
    }

    if !intersection_ty.is_null() {
        for &ty in unsafe { (*intersection_ty).parts.iter() } {
            if unsafe { get_type_id::<TableType>(ty) }.is_null() {
                return None;
            }
        }
    }

    let mut clone_state = CloneState {
        builtin_types,
        seen_types: DenseHashMap::new(core::ptr::null()),
        seen_type_packs: DenseHashMap::new(core::ptr::null()),
    };
    let result_type = shallow_clone(
        input_type,
        arena,
        &mut clone_state,
        /* clonePersistentTypes */ false,
    );

    let cloned_type_pack = arena.add_type_pack_t(TypePack {
        head: vec![result_type],
        tail: None,
    });
    Some(WithPredicate::with_predicate_t(cloned_type_pack))
}
