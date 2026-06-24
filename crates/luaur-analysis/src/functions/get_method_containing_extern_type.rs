use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::return_first_nonnull_option_of_type::return_first_nonnull_option_of_type;
use crate::records::extern_type::ExternType;
use crate::records::union_type::UnionType;
use crate::type_aliases::module_ptr_module::ModulePtr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::rtti;

pub fn get_method_containing_extern_type(
    module: &ModulePtr,
    func_expr: *mut AstExpr,
) -> Option<*const ExternType> {
    let parent_expr = if unsafe {
        (*func_expr).base.class_index == rtti::ast_rtti_index("AstExprIndexName")
    } {
        let index_name = unsafe { &*(func_expr as *mut AstExprIndexName) };
        index_name.expr
    } else if unsafe { (*func_expr).base.class_index == rtti::ast_rtti_index("AstExprIndexExpr") } {
        let index_expr = unsafe { &*(func_expr as *mut AstExprIndexExpr) };
        index_expr.expr
    } else {
        return None;
    };

    let parent_it = module.ast_types.find(&(parent_expr as *const AstExpr));
    let parent_it = *parent_it?;

    let parent_type = unsafe { follow_type_id(parent_it) };

    let extern_ptr = unsafe { get_type_id::<ExternType>(parent_type) };
    if !extern_ptr.is_null() {
        return Some(extern_ptr);
    }

    let union_ptr = unsafe { get_type_id::<UnionType>(parent_type) };
    if !union_ptr.is_null() {
        return unsafe { return_first_nonnull_option_of_type::<ExternType>(&*union_ptr) };
    }

    None
}
