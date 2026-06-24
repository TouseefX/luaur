// ConstraintGenerator::visitBlockWithoutChildScope (ConstraintGenerator.cpp:1276-1297).
use crate::enums::control_flow::ControlFlow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use core::mem::ManuallyDrop;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::DFInt;

impl ConstraintGenerator {
    pub fn visit_block_without_child_scope(
        &mut self,
        scope: *mut Scope,
        block: *mut AstStatBlock,
    ) -> ControlFlow {
        let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

        if self.recursion_count >= DFInt::LuauConstraintGeneratorRecursionLimit.get() as i32 {
            self.report_code_too_complex(unsafe { (*block).base.base.location });
            return ControlFlow::None;
        }

        self.prototype_type_definitions(scope, block);

        // Borrow the caller-owned `Scope` as a `ScopePtr` without taking ownership
        // (C++ passes `const ScopePtr&`); ManuallyDrop keeps the refcount intact.
        let scope_ptr: ManuallyDrop<ScopePtr> =
            ManuallyDrop::new(unsafe { alloc::sync::Arc::from_raw(scope as *const Scope) });

        let mut first_control_flow: Option<ControlFlow> = None;
        let body = unsafe { (*block).body };
        for i in 0..body.size {
            let stat = unsafe { *body.data.add(i) };
            let cf = self.visit_scope_ptr_ast_stat(&scope_ptr, stat);
            if cf != ControlFlow::None && first_control_flow.is_none() {
                first_control_flow = Some(cf);
            }
        }

        first_control_flow.unwrap_or(ControlFlow::None)
    }
}
