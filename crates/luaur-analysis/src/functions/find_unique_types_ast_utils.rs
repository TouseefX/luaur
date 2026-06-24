//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AstUtils.cpp:70:find_unique_types`
//! Source: `Analysis/src/AstUtils.cpp`
//!
//! Faithful port of:
//! ```cpp
//! void findUniqueTypes(NotNull<DenseHashSet<TypeId>> uniqueTypes, AstExpr* expr,
//!     NotNull<const DenseHashMap<const AstExpr*, TypeId>> astTypes)
//! {
//!     AstExprTableFinder finder{uniqueTypes, astTypes};
//!     expr->visit(&finder);
//! }
//! ```

use crate::records::ast_expr_table_finder::AstExprTableFinder;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::visit::ast_expr_visit;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub unsafe fn find_unique_types(
    unique_types: *mut DenseHashSet<TypeId>,
    expr: *mut AstExpr,
    ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
) {
    let mut finder = AstExprTableFinder::new(unique_types, ast_types);
    ast_expr_visit(expr, &mut finder);
}
