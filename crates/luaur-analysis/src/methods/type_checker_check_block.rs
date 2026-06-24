use crate::enums::control_flow::ControlFlow;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::recursion_limit_exception::RecursionLimitException;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::FInt;

impl TypeChecker {
    pub fn check_block(&mut self, scope: &ScopePtr, block: &AstStatBlock) -> ControlFlow {
        let _rc = RecursionCounter::recursion_counter_i32(&mut self.check_recursion_count);
        let limit = FInt::LuauCheckRecursionLimit.get();
        if limit > 0 && self.check_recursion_count >= limit {
            self.report_error_code_too_complex(&block.base.base.location);
            return ControlFlow::None;
        }

        match self.check_block_without_recursion_check(scope, block) {
            flow @ _ => flow,
        }
    }
}
