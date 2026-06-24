use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::recursion_counter::RecursionCounter;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_expr_table::Item;
use luaur_common::FFlag;
use luaur_common::FInt;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_table(&mut self, table: *mut AstExprTable) -> NonStrictContext {
        unsafe {
            let mut _rc = std::option::Option::None;
            if FFlag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
                _rc = std::option::Option::Some(RecursionCounter::recursion_counter_i32(
                    &mut self.non_strict_recursion_count,
                ));
                if FInt::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
                    && self.non_strict_recursion_count
                        >= FInt::LuauNonStrictTypeCheckerRecursionLimit.get()
                {
                    return NonStrictContext::non_strict_context();
                }
            }

            let items: AstArray<Item> = (*table).items.clone();
            for i in 0..items.size {
                let item: &Item = unsafe { &*items.data.add(i) };
                if !item.key.is_null() {
                    self.visit_ast_expr_value_context(item.key, ValueContext::RValue);
                }
                self.visit_ast_expr_value_context(item.value, ValueContext::RValue);
            }

            NonStrictContext::non_strict_context()
        }
    }
}
