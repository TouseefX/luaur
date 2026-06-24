use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;

impl AutocompleteNodeFinder {
    pub fn visit_ast_stat(&mut self, stat: *mut AstStat) -> bool {
        let stat_ref = unsafe { &*stat };

        if stat_ref.base.location.begin < self.pos
            && (stat_ref
                .has_semicolon
                .then(|| self.pos < stat_ref.base.location.end)
                .unwrap_or_else(|| self.pos <= stat_ref.base.location.end))
        {
            self.ancestry.push(stat as *mut AstNode);
            return true;
        }

        false
    }
}
