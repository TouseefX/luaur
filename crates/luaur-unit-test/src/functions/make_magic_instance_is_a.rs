//! Faithful Rust port of the test-only `struct MagicInstanceIsA : MagicFunction`
//! from `tests/TypeInfer.refinements.test.cpp` (lines 21-74).
//!
//! In C++ this is a `MagicFunction` subclass overriding `handleOldSolver`,
//! `infer`, and `refine`. In this port a `MagicFunction` *is* its vtable (a set
//! of function pointers), so the "subclass" is simply a `MagicFunction` built
//! from the three handler functions below. `make_magic_instance_is_a` returns
//! the shared instance that the refinement fixture attaches to the `IsA`
//! function type.

use alloc::string::String;
use alloc::sync::Arc;

use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
use luaur_analysis::functions::try_get_l_value::try_get_l_value;
use luaur_analysis::records::is_a_predicate::IsAPredicate;
use luaur_analysis::records::magic_function::MagicFunction;
use luaur_analysis::records::magic_function_call_context::MagicFunctionCallContext;
use luaur_analysis::records::magic_function_type_check_context::MagicFunctionTypeCheckContext;
use luaur_analysis::records::magic_refinement_context::MagicRefinementContext;
use luaur_analysis::records::module::Module;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::type_checker::TypeChecker;
use luaur_analysis::records::with_predicate::WithPredicate;
use luaur_analysis::type_aliases::predicate::Predicate;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_analysis::type_aliases::type_variant::TypeVariant;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

/// C++ `std::string(str->value.data, str->value.size)`. The string constant's
/// bytes are stored as an `AstArray<c_char>`; reconstruct the owned `String`.
fn ast_string_value(value: &AstArray<core::ffi::c_char>) -> String {
    let slice = value.as_slice();
    let mut s = String::with_capacity(slice.len());
    for &c in slice {
        s.push(c as u8 as char);
    }
    s
}

/// C++ `MagicInstanceIsA::handleOldSolver` (old type checker path).
fn magic_instance_is_a_handle_old_solver(
    type_checker: &mut TypeChecker,
    scope: &Arc<Scope>,
    expr: &AstExprCall,
    _with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    if expr.args.size != 1 {
        return None;
    }

    let index = unsafe { ast_node_as::<AstExprIndexName>(expr.func as *mut AstNode) };
    let args = expr.args.as_slice();
    let str_node = if args.is_empty() {
        core::ptr::null_mut()
    } else {
        unsafe { ast_node_as::<AstExprConstantString>(args[0] as *mut AstNode) }
    };
    if index.is_null() || str_node.is_null() {
        return None;
    }

    let lvalue = unsafe { try_get_l_value(&*(*index).expr) };
    let name = ast_string_value(unsafe { &(*str_node).value });
    let tfun = scope.lookup_type(&name);

    let (lvalue, tfun) = match (lvalue, tfun) {
        (Some(l), Some(t)) => (l, t),
        _ => return None,
    };

    let module_arc = type_checker.current_module.clone()?;
    let module_ptr = Arc::as_ptr(&module_arc) as *mut Module;
    let boolean_pack = unsafe {
        (*module_ptr)
            .internal_types
            .add_type_pack_initializer_list_type_id(&[type_checker.boolean_type])
    };

    Some(WithPredicate::with_predicate_t_predicate_vec(
        boolean_pack,
        alloc::vec![Predicate::IsA(IsAPredicate {
            lvalue,
            location: expr.base.base.location,
            ty: tfun.r#type(),
        })],
    ))
}

/// C++ `MagicInstanceIsA::infer` — returns false (this magic does not infer).
fn magic_instance_is_a_infer(_context: &MagicFunctionCallContext) -> bool {
    false
}

/// C++ `MagicInstanceIsA::refine` (new constraint-solver path).
fn magic_instance_is_a_refine(ctx: &MagicRefinementContext) {
    let call_site = unsafe { &*ctx.call_site };

    if call_site.args.size != 1 || ctx.discriminant_types.is_empty() {
        return;
    }

    let index = unsafe { ast_node_as::<AstExprIndexName>(call_site.func as *mut AstNode) };
    let args = call_site.args.as_slice();
    let str_node = if args.is_empty() {
        core::ptr::null_mut()
    } else {
        unsafe { ast_node_as::<AstExprConstantString>(args[0] as *mut AstNode) }
    };
    if index.is_null() || str_node.is_null() {
        return;
    }

    let discriminant_ty = match ctx.discriminant_types[0] {
        Some(ty) => ty,
        None => return,
    };

    let name = ast_string_value(unsafe { &(*str_node).value });
    let scope = unsafe { &*ctx.scope };
    let tfun = match scope.lookup_type(&name) {
        Some(tfun) => tfun,
        None => return,
    };

    // C++: LUAU_ASSERT(get<BlockedType>(*discriminantTy));
    //      asMutable(*discriminantTy)->ty.emplace<BoundType>(tfun->type);
    let mutable_ty = as_mutable_type_id(discriminant_ty);
    unsafe {
        (*mutable_ty).ty = TypeVariant::Bound(tfun.r#type());
    }
}

fn magic_instance_is_a_type_check(_context: &MagicFunctionTypeCheckContext) -> bool {
    false
}

/// Build the shared `MagicInstanceIsA` instance (C++ `std::make_shared<MagicInstanceIsA>()`).
pub fn make_magic_instance_is_a() -> Arc<MagicFunction> {
    Arc::new(MagicFunction::from_handlers(
        magic_instance_is_a_handle_old_solver,
        magic_instance_is_a_infer,
        magic_instance_is_a_refine,
        magic_instance_is_a_type_check,
    ))
}
