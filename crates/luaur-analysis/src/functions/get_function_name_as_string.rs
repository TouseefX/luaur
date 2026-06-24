use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;

pub fn get_function_name_as_string(expr: &AstExpr) -> Option<alloc::string::String> {
    let mut curr = expr as *const AstExpr;
    let mut s = alloc::string::String::new();

    unsafe {
        loop {
            let local = luaur_ast::rtti::ast_node_as::<AstExprLocal>(
                curr as *mut luaur_ast::records::ast_node::AstNode,
            );
            if !local.is_null() {
                let name_ptr = (*(*local).local).name.value;
                let mut name = if name_ptr.is_null() {
                    alloc::string::String::new()
                } else {
                    core::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .into_owned()
                };
                name.push_str(&s);
                return Some(name);
            }

            let global = luaur_ast::rtti::ast_node_as::<AstExprGlobal>(
                curr as *mut luaur_ast::records::ast_node::AstNode,
            );
            if !global.is_null() {
                let name_ptr = (*global).name.value;
                let mut name = if name_ptr.is_null() {
                    alloc::string::String::new()
                } else {
                    core::ffi::CStr::from_ptr(name_ptr)
                        .to_string_lossy()
                        .into_owned()
                };
                name.push_str(&s);
                return Some(name);
            }

            let indexname = luaur_ast::rtti::ast_node_as::<AstExprIndexName>(
                curr as *mut luaur_ast::records::ast_node::AstNode,
            );
            if !indexname.is_null() {
                curr = (*indexname).expr;

                let index_ptr = (*indexname).index.value;
                let index_str = if index_ptr.is_null() {
                    alloc::string::String::new()
                } else {
                    core::ffi::CStr::from_ptr(index_ptr)
                        .to_string_lossy()
                        .into_owned()
                };

                let mut new_s = alloc::string::String::new();
                new_s.push('.');
                new_s.push_str(&index_str);
                new_s.push_str(&s);
                s = new_s;

                continue;
            }

            let group = luaur_ast::rtti::ast_node_as::<AstExprGroup>(
                curr as *mut luaur_ast::records::ast_node::AstNode,
            );
            if !group.is_null() {
                curr = (*group).expr;
                continue;
            }

            return None;
        }
    }
}
