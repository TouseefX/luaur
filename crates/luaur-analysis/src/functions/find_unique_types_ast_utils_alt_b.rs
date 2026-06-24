//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AstUtils.cpp:76:find_unique_types`
//! Source: `Analysis/src/AstUtils.cpp`
//!
//! Faithful port of the `Iter` function template:
//! ```cpp
//! template<typename Iter>
//! void findUniqueTypes(NotNull<DenseHashSet<TypeId>> uniqueTypes, Iter startIt, Iter endIt,
//!     NotNull<const DenseHashMap<const AstExpr*, TypeId>> astTypes)
//! {
//!     while (startIt != endIt)
//!     {
//!         AstExpr* expr = *startIt;
//!         if (expr->is<AstExprTable>())
//!             findUniqueTypes(uniqueTypes, expr, astTypes);
//!         ++startIt;
//!     }
//! }
//! ```
//! The C++ `Iter` template is rendered as a Rust iterator over `*mut AstExpr`;
//! the begin/end iterator pair becomes a single `Iterator` argument (the callers
//! at L94/L103 pass `exprs.iter()`).

use crate::functions::find_unique_types_ast_utils::find_unique_types;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::rtti::ast_node_is;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub unsafe fn find_unique_types_iter<I>(
    unique_types: *mut DenseHashSet<TypeId>,
    iter: I,
    ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
) where
    I: IntoIterator<Item = *mut AstExpr>,
{
    for expr in iter {
        if ast_node_is::<AstExprTable>(&(*expr).base) {
            find_unique_types(unique_types, expr, ast_types);
        }
    }
}
