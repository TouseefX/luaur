//! @interface-stub
use crate::records::magic_instance_is_a::MagicInstanceIsA;
use luaur_analysis::records::type_checker::TypeChecker;
use luaur_analysis::records::with_predicate::WithPredicate;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl MagicInstanceIsA {
    pub fn handle_old_solver(
        &mut self,
        _type_checker: &mut TypeChecker,
        _scope: *mut core::ffi::c_void,
        _expr: &AstExprCall,
        _with_predicate: WithPredicate<TypePackId>,
    ) -> Option<WithPredicate<TypePackId>> {
        // Dead duplicate skeleton node: the C++ `MagicInstanceIsA` subclass became
        // a vtable-built `MagicFunction`; the real handleOldSolver logic is the
        // free `magic_instance_is_a_handle_old_solver` in
        // `crates/luau-unit-test/src/functions/make_magic_instance_is_a.rs`. This
        // method (different signature) has no call site.
        unreachable!("canonical handler lives in functions/make_magic_instance_is_a.rs (vtable-built MagicFunction); this skeleton node is unused");
    }
}
