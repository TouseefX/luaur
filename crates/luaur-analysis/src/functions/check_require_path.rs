use crate::records::deprecated_api_used::DeprecatedApiUsed;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::type_error_data::TypeErrorData;
use alloc::string::ToString;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn check_require_path(typechecker: &mut TypeChecker, expr: *mut AstExpr) -> bool {
    let mut good = true;
    let mut index_expr = unsafe { ast_node_as::<AstExprIndexName>(expr as *mut AstNode) };

    while !index_expr.is_null() {
        let index_ref = unsafe { &*index_expr };
        let index_bytes = unsafe { core::ffi::CStr::from_ptr(index_ref.index.value).to_bytes() };

        if index_bytes == b"parent" {
            typechecker.report_error_location_type_error_data(
                &index_ref.index_location,
                TypeErrorData::DeprecatedApiUsed(DeprecatedApiUsed {
                    symbol: "parent".to_string(),
                    use_instead: "Parent".to_string(),
                }),
            );
            good = false;
        }

        index_expr = unsafe { ast_node_as::<AstExprIndexName>(index_ref.expr as *mut AstNode) };
    }

    good
}
