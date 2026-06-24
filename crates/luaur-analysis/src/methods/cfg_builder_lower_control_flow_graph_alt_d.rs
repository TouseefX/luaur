use crate::records::assign::Assign;
use crate::records::cfg_builder::CfgBuilder;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl CfgBuilder {
    pub fn lower_ast_stat_assign(&mut self, assn: *mut AstStatAssign) {
        unsafe {
            for i in 0..(*assn).values.size {
                let expr = *(*assn).values.data.add(i as usize);
                self.lower_expr_ast_expr(expr);
            }

            for i in 0..(*assn).vars.size {
                let target = *(*assn).vars.data.add(i as usize);

                // C++:
                //   if (auto sym = extractLValueSymbol(target)) {
                //       DefId def = newDefinition(*sym);
                //       emit<Assign>(currentBlock, def, assn);
                //       currentBlock->setReachingDefinition(*sym, def);
                //   } else LUAU_ASSERT(!"Unhandled lvalue type");
                if let Some(sym) =
                    crate::functions::extract_l_value_symbol::extract_l_value_symbol(&*target)
                {
                    let def = self.new_definition(sym.clone());
                    let current_block = self.current_block;
                    self.emit::<Assign, _>(current_block, (def, assn));
                    (*current_block).set_reaching_definition(sym, def);
                } else {
                    LUAU_ASSERT!(false);
                }
            }
        }
    }
}
