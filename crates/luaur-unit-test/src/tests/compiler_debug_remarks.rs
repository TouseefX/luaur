#[cfg(test)]
#[test]
fn compiler_debug_remarks() {
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_REMARKS);

    let fid = bcb.begin_function(0, false);

    bcb.add_debug_remark(core::format_args!("test remark #1"));
    bcb.emit_abc(LuauOpcode::LOP_LOADNIL, 0, 0, 0);
    bcb.add_debug_remark(core::format_args!("test remark #2"));
    bcb.add_debug_remark(core::format_args!("test remark #3"));
    bcb.emit_abc(LuauOpcode::LOP_RETURN, 0, 1, 0);

    bcb.end_function(1, 0, 0);

    bcb.set_main_function(fid);
    bcb.finalize();

    let dump_func = bcb.dump_function(0);
    let expected_func = "\nREMARK test remark #1\nLOADNIL R0\nREMARK test remark #2\nREMARK test remark #3\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
