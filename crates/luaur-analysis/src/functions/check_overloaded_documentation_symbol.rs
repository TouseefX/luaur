use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::module::Module;
use crate::type_aliases::documentation_symbol::DocumentationSymbol;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti;

pub fn check_overloaded_documentation_symbol(
    module: &Module,
    ty: TypeId,
    parent_expr: *const AstExpr,
    documentation_symbol: Option<DocumentationSymbol>,
) -> Option<DocumentationSymbol> {
    let documentation_symbol = documentation_symbol?;

    let follow_ty = unsafe { follow_type_id(ty) };
    let intersection_ptr = unsafe { get_type_id::<IntersectionType>(follow_ty) };
    if intersection_ptr.is_null() {
        return Some(documentation_symbol);
    }

    let matching_overload = if !parent_expr.is_null()
        && unsafe { (*parent_expr).base.class_index == rtti::ast_rtti_index("AstExprCall") }
    {
        let node_ptr = parent_expr as *const AstNode;
        module.ast_overload_resolved_types.find(&node_ptr).copied()
    } else {
        None
    };

    if let Some(matching_overload) = matching_overload {
        let mut overload_symbol = documentation_symbol;
        overload_symbol.push_str("/overload/");
        let ty_str = to_string_type_id(matching_overload);
        overload_symbol.push_str(&ty_str);
        Some(overload_symbol)
    } else {
        Some(documentation_symbol)
    }
}
