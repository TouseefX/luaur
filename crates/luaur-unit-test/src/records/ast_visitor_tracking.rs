use luaur_ast::records::ast_node::AstNode;

#[derive(Debug, Clone)]
pub struct AstVisitorTracking {
    pub(crate) visited_nodes: Vec<*mut AstNode>,
    pub(crate) seen: std::collections::HashSet<usize>,
}
