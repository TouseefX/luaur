use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::check_type_match::check_type_match;
use crate::functions::find_expected_type_at::find_expected_type_at;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

/// C++ `static TypeCorrectKind checkTypeCorrectKind(...)` (AutocompleteCore.cpp:209-258).
pub fn check_type_correct_kind(
    module: &Module,
    type_arena: *mut TypeArena,
    builtin_types: &BuiltinTypes,
    node: *mut AstNode,
    position: Position,
    ty: TypeId,
) -> TypeCorrectKind {
    let ty = unsafe { follow_type_id(ty) };

    LUAU_ASSERT!(module.has_module_scope());

    let module_scope = module.get_module_scope();
    let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

    let type_at_position = find_expected_type_at(module, node, position);

    let type_at_position = match type_at_position {
        Some(t) => t,
        None => return TypeCorrectKind::None,
    };

    let expected_type = unsafe { follow_type_id(type_at_position) };

    let builtin_types_ptr = builtin_types as *const BuiltinTypes as *mut BuiltinTypes;

    // `checkFunctionType` lambda from C++: suggest functions whose first return
    // type matches the expected type.
    let check_function_type = |ftv: &FunctionType| -> bool {
        if let Some(first_ret_ty) = first(ftv.ret_types, true) {
            return check_type_match(
                module,
                first_ret_ty,
                expected_type,
                module_scope_ptr,
                type_arena,
                builtin_types_ptr,
            );
        }
        false
    };

    // We also want to suggest functions that return compatible result
    let ftv = unsafe { get_type_id::<FunctionType>(ty) };
    if !ftv.is_null() && check_function_type(unsafe { &*ftv }) {
        return TypeCorrectKind::CorrectFunctionResult;
    } else {
        let itv = unsafe { get_type_id::<IntersectionType>(ty) };
        if !itv.is_null() {
            for &id in unsafe { &(*itv).parts } {
                let id = unsafe { follow_type_id(id) };

                let ftv = unsafe { get_type_id::<FunctionType>(id) };
                if !ftv.is_null() && check_function_type(unsafe { &*ftv }) {
                    return TypeCorrectKind::CorrectFunctionResult;
                }
            }
        }
    }

    if check_type_match(
        module,
        ty,
        expected_type,
        module_scope_ptr,
        type_arena,
        builtin_types_ptr,
    ) {
        TypeCorrectKind::Correct
    } else {
        TypeCorrectKind::None
    }
}
