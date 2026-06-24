use crate::records::lint_duplicate_function::LintDuplicateFunction;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::rtti::ast_node_as;

impl LintDuplicateFunction {
    pub fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
        self.defns.clear();

        let block_ref = unsafe { &*block };
        let body = block_ref.body;

        for i in 0..body.size {
            let stat = unsafe { *body.data.add(i) };
            let node = stat as *mut AstNode;

            unsafe {
                let func = ast_node_as::<AstStatFunction>(node);
                if !func.is_null() {
                    self.track_function(
                        (*(*func).name).base.location,
                        &self.build_name((*func).name),
                    );
                    continue;
                }

                let local_func = ast_node_as::<AstStatLocalFunction>(node);
                if !local_func.is_null() {
                    let name = (*(*local_func).name).name;
                    if !name.value.is_null() {
                        let name = core::ffi::CStr::from_ptr(name.value).to_string_lossy();
                        self.track_function((*(*local_func).name).location, &name);
                    }
                }
            }
        }

        true
    }
}
