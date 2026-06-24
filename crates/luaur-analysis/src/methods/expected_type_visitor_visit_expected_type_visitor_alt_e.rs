//! @interface-stub
use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::index_collector::IndexCollector;
use crate::records::union_type::UnionType;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;

impl ExpectedTypeVisitor {
    pub fn visit_ast_expr_index_expr(&mut self, expr: *mut AstExprIndexExpr) -> bool {
        unsafe {
            let expr_ref = &*expr;

            if let Some(&ty) = (*self.ast_types).find(&(expr_ref.expr as *const _)) {
                let mut ic = IndexCollector::new(self.arena);
                ic.traverse_type_id(ty);

                if ic.indexes.size() > 1 {
                    let union = (*self.arena).add_type(UnionType {
                        options: ic.indexes.take(),
                    });
                    self.apply_expected_type(union, expr_ref.index as *const _);
                } else if ic.indexes.size() == 1 {
                    let first = ic.indexes.order[0];
                    self.apply_expected_type(first, expr_ref.index as *const _);
                }
            }
        }

        true
    }
}
