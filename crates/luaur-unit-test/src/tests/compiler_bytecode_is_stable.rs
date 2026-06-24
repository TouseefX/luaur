#[cfg(test)]
#[test]
fn compiler_bytecode_is_stable() {
    use luaur_common::enums::luau_builtin_function::LuauBuiltinFunction as LBF;
    use luaur_common::enums::luau_bytecode_tag::LuauBytecodeTag as LBC;
    use luaur_common::enums::luau_bytecode_type::LuauBytecodeType as LBC_TYPE;
    use luaur_common::enums::luau_capture_type::LuauCaptureType as LCT;
    use luaur_common::enums::luau_opcode::LuauOpcode as LOP;

    // Bytecode ops (serialized & in-memory)
    assert_eq!(LOP::LOP_FASTCALL2K as i32, 75); // bytecode v1
    assert_eq!(LOP::LOP_JUMPXEQKS as i32, 80); // bytecode v3

    // Bytecode fastcall ids (serialized & in-memory)
    // Note: these aren't strictly bound to specific bytecode versions, but must monotonically increase to keep backwards compat
    assert_eq!(LBF::LBF_VECTOR as i32, 54);
    assert_eq!(LBF::LBF_TOSTRING as i32, 63);
    assert_eq!(LBF::LBF_BUFFER_WRITEF64 as i32, 77);
    assert_eq!(LBF::LBF_VECTOR_MAX as i32, 88);

    // Bytecode capture type (serialized & in-memory)
    assert_eq!(LCT::LCT_UPVAL as i32, 2); // bytecode v1

    // Bytecode constants (serialized)
    assert_eq!(LBC::LBC_CONSTANT_CLOSURE.0 as i32, 6); // bytecode v1

    // Bytecode type encoding (serialized & in-memory)
    // Note: these *can* change retroactively *if* type version is bumped, but probably shouldn't
    assert_eq!(LBC_TYPE::LBC_TYPE_BUFFER.0 as i32, 9); // type version 1
}
