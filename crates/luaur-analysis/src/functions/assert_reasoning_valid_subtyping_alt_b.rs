use crate::records::builtin_types::BuiltinTypes;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_indexer::TableIndexer;
use crate::records::type_arena::TypeArena;

pub fn assert_reasoning_valid(
    sub_ty: TableIndexer,
    super_ty: TableIndexer,
    result: &SubtypingResult,
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
) {
}
