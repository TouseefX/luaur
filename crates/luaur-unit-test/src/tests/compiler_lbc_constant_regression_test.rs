#[cfg(test)]
#[test]
fn compiler_lbc_constant_regression_test() {
    use luaur_common::enums::luau_bytecode_tag::LuauBytecodeTag;

    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_NIL.0, 0);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0, 1);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_NUMBER.0, 2);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_STRING.0, 3);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_IMPORT.0, 4);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_TABLE.0, 5);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0, 6);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_VECTOR.0, 7);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0, 8);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_INTEGER.0, 9);
    assert_eq!(LuauBytecodeTag::LBC_CONSTANT_CLASS_SHAPE.0, 10);

    assert_eq!(LuauBytecodeTag::LBC_CONSTANT__COUNT.0, 11);
}
