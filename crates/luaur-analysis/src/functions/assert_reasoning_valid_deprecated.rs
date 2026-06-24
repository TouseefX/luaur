use crate::records::builtin_types::BuiltinTypes;
use crate::records::subtyping_result::SubtypingResult;

pub fn assert_reasoning_valid_deprecated<TID>(
    sub_ty: TID,
    super_ty: TID,
    result: &SubtypingResult,
    builtin_types: *mut BuiltinTypes,
) {
    if !luaur_common::FFlag::DebugLuauSubtypingCheckPathValidity.get() {
        return;
    }
    for reasoning in result.reasoning.iter() {
        // LUAU_ASSERT!(traverse_deprecated(sub_ty, reasoning.sub_path, builtin_types));
        // LUAU_ASSERT!(traverse_deprecated(super_ty, reasoning.super_path, builtin_types));
    }
}
