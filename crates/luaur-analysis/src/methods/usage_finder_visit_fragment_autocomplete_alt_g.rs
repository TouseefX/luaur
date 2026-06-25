use crate::records::symbol::Symbol;
use crate::records::usage_finder::UsageFinder;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl UsageFinder {
    // C++ `bool UsageFinder::visit(AstExprGlobal* global)` (FragmentAutocomplete.cpp:641-647):
    //   globalDefsToPrePopulate.emplace_back(global->name, dfg->getDef(global));
    //   auto def = dfg->getDef(global);
    //   symbolsToRefine.emplace_back(def, Symbol(global->name));
    //   return true;
    pub fn visit_ast_expr_global(&mut self, global: *mut AstExprGlobal) -> bool {
        let dfg = unsafe { &*self.dfg };
        let name = unsafe { (*global).name };
        let def = dfg.get_def(global as *const AstExpr);

        self.global_defs_to_pre_populate.push((name, def));
        self.symbols_to_refine
            .push((def, Symbol::from_global(name)));

        true
    }
}
