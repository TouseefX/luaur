use crate::functions::find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position_source_module_position_bool;
use crate::records::binding::Binding;
use crate::records::source_module::SourceModule;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;

pub fn find_binding_local_statement(
    source: &SourceModule,
    binding: &Binding,
) -> Option<*mut AstStatLocal> {
    if binding.location.begin == Position::new(0, 0) && binding.location.end == Position::new(0, 0)
    {
        return None;
    }

    let nodes = find_ast_ancestry_of_position_source_module_position_bool(
        source,
        binding.location.begin,
        false,
    );

    let mut iter = nodes.iter().rev();
    while let Some(&node) = iter.next() {
        let stat_local = unsafe { ast_node_as::<AstStatLocal>(node as *mut AstNode) };
        if !stat_local.is_null() {
            return Some(stat_local);
        }
    }

    None
}
