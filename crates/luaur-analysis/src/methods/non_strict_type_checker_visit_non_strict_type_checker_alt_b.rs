use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::stack_pusher_non_strict_type_checker::StackPusher;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;
use luaur_common::FInt;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> NonStrictContext {
        LUAU_ASSERT!(!block.is_null());

        let mut _rc: Option<RecursionCounter> = None;
        if FFlag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
            _rc = Some(RecursionCounter::recursion_counter_i32(
                &mut self.non_strict_recursion_count,
            ));
            if FInt::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
                && self.non_strict_recursion_count
                    >= FInt::LuauNonStrictTypeCheckerRecursionLimit.get()
            {
                return NonStrictContext::non_strict_context();
            }
        }

        let _stack_pusher = self.push_stack(block as *mut luaur_ast::records::ast_node::AstNode);

        let mut ctx = NonStrictContext::non_strict_context();

        unsafe {
            let block = &*block;
            let mut i = block.body.size;
            while i > 0 {
                i -= 1;
                let stat = *block.body.data.add(i);

                let local_ptr = luaur_ast::rtti::ast_node_as::<AstStatLocal>(
                    stat as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !local_ptr.is_null() {
                    let local = &*local_ptr;
                    self.visit_ast_stat(stat);
                    let mut j = 0usize;
                    while j < local.vars.size {
                        let var = *local.vars.data.add(j);
                        let def = (*self.dfg).get_def_ast_local(var);
                        ctx.remove(&def);
                        // C++ `visit(local->annotation)` — `local` here is the loop var (AstLocal).
                        self.visit_ast_type((*var).annotation);
                        j += 1;
                    }
                } else {
                    let other_ctx = self.visit_ast_stat(stat);
                    ctx = NonStrictContext::disjunction(
                        self.builtin_types,
                        self.arena,
                        &other_ctx,
                        &ctx,
                    );
                }
            }
        }

        ctx
    }
}
