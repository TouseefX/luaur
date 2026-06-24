use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;
use luaur_config::enums::code::Code;

use crate::functions::emit_warning::emit_warning;
use crate::records::lint_context::LintContext;

#[derive(Debug, Clone, Default)]
pub struct Global {
    pub(crate) location: Location,
    pub(crate) function: bool,
    pub(crate) used: bool,
}

impl DenseDefault for Global {
    fn dense_default() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct LintUnusedFunction {
    pub(crate) context: *mut LintContext,
    pub(crate) globals: DenseHashMap<AstName, Global>,
}

impl LintUnusedFunction {
    pub fn new() -> Self {
        Self {
            context: core::ptr::null_mut(),
            globals: DenseHashMap::new(AstName::new()),
        }
    }

    pub fn process(&mut self, context: &mut LintContext) {
        self.context = context as *mut LintContext;
        // SAFETY: The visitor pattern requires a valid AST root.
        // The context is guaranteed to be valid for the duration of the visit.
        unsafe {
            let root = (*self.context).root;
            luaur_ast::visit::ast_stat_visit(root, self);
        }
        self.report();
    }

    pub fn report(&mut self) {
        for (name, global) in self.globals.iter() {
            if global.function && !global.used {
                let c_str = name.value;
                if !c_str.is_null() {
                    let first_char = unsafe { *c_str };
                    if first_char != b'_' as i8 {
                        let name = unsafe { core::ffi::CStr::from_ptr(c_str).to_string_lossy() };
                        emit_warning(
                            unsafe { &mut *self.context },
                            Code::Code_FunctionUnused,
                            global.location,
                            format_args!(
                                "Function '{}' is never used; prefix with '_' to silence",
                                name
                            ),
                        );
                    }
                }
            }
        }
    }

    pub fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstStatFunction;
        unsafe {
            let name_expr = (*node).name;
            let expr = luaur_ast::rtti::ast_node_as::<AstExprGlobal>(name_expr as *mut AstNode);
            if !expr.is_null() {
                let g = self.globals.get_or_insert((*expr).name);
                g.function = true;
                g.location = (*expr).base.base.location;
                luaur_ast::visit::ast_expr_visit(
                    (*node).func as *mut luaur_ast::records::ast_expr::AstExpr,
                    self,
                );
                return false;
            }
        }
        true
    }

    pub fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        let node = node as *mut AstExprGlobal;
        unsafe {
            let g = self.globals.get_or_insert((*node).name);
            g.used = true;
        }
        true
    }
}

impl AstVisitor for LintUnusedFunction {
    fn visit_stat_function(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_stat_function(node)
    }

    fn visit_expr_global(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_expr_global(node)
    }

    fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }

    fn visit_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }

    fn visit_type_pack(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }
}
