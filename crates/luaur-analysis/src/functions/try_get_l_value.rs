use crate::records::field::Field;
use crate::records::symbol::Symbol;
use crate::type_aliases::l_value::LValue;
use alloc::sync::Arc;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::rtti::ast_node_as;

pub fn try_get_l_value(node: &AstExpr) -> Option<LValue> {
    let mut expr = node as *const AstExpr;

    unsafe {
        loop {
            let group =
                ast_node_as::<AstExprGroup>(expr as *mut luaur_ast::records::ast_node::AstNode);
            if !group.is_null() {
                expr = (*group).expr;
                continue;
            }
            break;
        }

        let local = ast_node_as::<AstExprLocal>(expr as *mut luaur_ast::records::ast_node::AstNode);
        if !local.is_null() {
            return Some(LValue::Symbol(Symbol::symbol_ast_local((*local).local)));
        }

        let global =
            ast_node_as::<AstExprGlobal>(expr as *mut luaur_ast::records::ast_node::AstNode);
        if !global.is_null() {
            return Some(LValue::Symbol(Symbol::symbol_ast_name((*global).name)));
        }

        let indexname =
            ast_node_as::<AstExprIndexName>(expr as *mut luaur_ast::records::ast_node::AstNode);
        if !indexname.is_null() {
            if let Some(lvalue) = try_get_l_value(&*(*indexname).expr) {
                let key = if (*indexname).index.value.is_null() {
                    alloc::string::String::new()
                } else {
                    core::ffi::CStr::from_ptr((*indexname).index.value)
                        .to_string_lossy()
                        .into_owned()
                };
                return Some(LValue::Field(Field {
                    parent: Some(Arc::new(lvalue)),
                    key,
                }));
            }
        }

        let indexexpr =
            ast_node_as::<AstExprIndexExpr>(expr as *mut luaur_ast::records::ast_node::AstNode);
        if !indexexpr.is_null() {
            if let Some(lvalue) = try_get_l_value(&*(*indexexpr).expr) {
                let string_node = ast_node_as::<AstExprConstantString>(
                    (*indexexpr).index as *mut luaur_ast::records::ast_node::AstNode,
                );
                if !string_node.is_null() {
                    let data = (*string_node).value.data;
                    let size = (*string_node).value.size;
                    let key = if data.is_null() {
                        alloc::string::String::new()
                    } else {
                        let slice = core::slice::from_raw_parts(data as *const u8, size as usize);
                        alloc::string::String::from_utf8_lossy(slice).into_owned()
                    };
                    return Some(LValue::Field(Field {
                        parent: Some(Arc::new(lvalue)),
                        key,
                    }));
                }
            }
        }
    }

    None
}
