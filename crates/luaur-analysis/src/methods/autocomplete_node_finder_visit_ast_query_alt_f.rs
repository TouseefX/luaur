use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_type_error::AstTypeError;

impl AutocompleteNodeFinder {
    pub fn visit_ast_stat_block(&mut self, block: *mut AstStatBlock) -> bool {
        // If ancestry is empty, we are inspecting the root of the AST.  Its extent is considered to be infinite.
        if self.ancestry.is_empty() {
            self.ancestry.push(block as *mut AstNode);
            return true;
        }

        unsafe {
            let last = *self.ancestry.last().unwrap();

            // AstExprIndexName nodes are nested outside-in, so we want the outermost node in the case of nested nodes.
            // ex foo.bar.baz is represented in the AST as IndexName{ IndexName {foo, bar}, baz}
            if (*last).is::<AstExprIndexName>() {
                return false;
            }

            // Type annotation error might intersect the block statement when the function header is being written,
            // annotation takes priority
            if (*last).is::<AstTypeError>() {
                return false;
            }

            let location = (*block).base.base.location;

            // If the cursor is at the end of an expression or type and simultaneously at the beginning of a block,
            // the expression or type wins out.
            // The exception to this is if we are in a block under an AstExprFunction.  In this case, we consider the position to
            // be within the block.
            if location.begin == self.pos {
                if !(*last).as_expr_const().is_null() && !(*last).is::<AstExprFunction>() {
                    return false;
                }

                if !(*last).as_type().is_null() {
                    return false;
                }
            }

            if location.begin <= self.pos && self.pos <= location.end {
                self.ancestry.push(block as *mut AstNode);
                return true;
            }
        }
        false
    }
}
