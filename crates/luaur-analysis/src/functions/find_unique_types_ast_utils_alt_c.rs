//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AstUtils.cpp:94:find_unique_types`
//! Source: `Analysis/src/AstUtils.cpp`
//!
//! Faithful port of:
//! ```cpp
//! void findUniqueTypes(NotNull<DenseHashSet<TypeId>> uniqueTypes, AstArray<AstExpr*> exprs,
//!     NotNull<const DenseHashMap<const AstExpr*, TypeId>> astTypes)
//! {
//!     findUniqueTypes(uniqueTypes, exprs.begin(), exprs.end(), astTypes);
//! }
//! ```

use crate::functions::find_unique_types_ast_utils_alt_b::find_unique_types_iter;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub unsafe fn find_unique_types(
    unique_types: *mut DenseHashSet<TypeId>,
    exprs: AstArray<*mut AstExpr>,
    ast_types: *const DenseHashMap<*const AstExpr, TypeId>,
) {
    find_unique_types_iter(unique_types, exprs.iter().copied(), ast_types);
}
