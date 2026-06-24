//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:4938:compiler_jump_trampoline`
//! Source: `tests/Compiler.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Compiler.test.cpp
//! - source_includes:
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Compiler.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record BytecodeBuilder (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - calls -> method BytecodeBuilder::setDumpFlags (Bytecode/include/Luau/BytecodeBuilder.h)
//!   - type_ref -> record CompileOptions (Compiler/include/Luau/Compiler.h)
//!   - calls -> method BytecodeBuilder::dumpFunction (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item compiler_jump_trampoline

#[cfg(test)]
#[test]
fn compiler_jump_trampoline() {
    use alloc::string::String;
    use alloc::vec::Vec;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let mut source = String::new();
    source.push_str("local sum: number = 0\n");
    source.push_str("for i=1,3 do\n");
    for _ in 0..10000 {
        source.push_str("sum = sum + i\n");
        source.push_str("if sum > 150000 then break end\n");
    }
    source.push_str("end\n");
    source.push_str("return sum\n");

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(
        BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LOCALS | BytecodeBuilder::DUMP_TYPES,
    );

    let mut options = luaur_compiler::records::compile_options::CompileOptions::default();
    options.debug_level = 2;
    options.type_info_level = 1;

    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump = bcb.dump_function(0);

    // mirror C++ std::getline over the stringstream: split into lines, dropping
    // the trailing empty element after the final newline.
    let insns: Vec<&str> = dump.lines().collect();

    // FORNPREP and early JUMPs (break) need to go through a trampoline
    let mut head = String::new();
    for i in 0..20 {
        head.push_str(insns[i]);
        head.push('\n');
    }

    let expected_head = r#"
local 0: reg 3, start pc 8 line 3, end pc 54545 line 20002
local 1: reg 0, start pc 2 line 2, end pc 54549 line 20004
R3: number from 2 to 54546
R0: number from 1 to 54550
LOADN R0 0
LOADN R3 1
LOADN R1 3
LOADN R2 1
JUMP L1
L0: JUMPX L14543
L1: FORNPREP R1 L0
L2: ADD R0 R0 R3
LOADK R4 K0 [150000]
JUMP L4
L3: JUMPX L14543
L4: JUMPIFLT R4 R0 L3
ADD R0 R0 R3
LOADK R4 K0 [150000]
JUMP L6
L5: JUMPX L14543
"#;
    assert_eq!(format!("\n{}", head), expected_head);

    // FORNLOOP has to go through a trampoline since the jump is back to the beginning of the function
    // however, late JUMPs (break) don't need a trampoline since the loop end is really close by
    let mut tail = String::new();
    for i in 44543..insns.len() {
        tail.push_str(insns[i]);
        tail.push('\n');
    }

    let expected_tail = r#"
ADD R0 R0 R3
LOADK R4 K0 [150000]
JUMPIFLT R4 R0 L14543
ADD R0 R0 R3
LOADK R4 K0 [150000]
JUMPIFLT R4 R0 L14543
JUMP L14542
L14541: JUMPX L2
L14542: FORNLOOP R1 L14541
L14543: RETURN R0 1
"#;
    assert_eq!(format!("\n{}", tail), expected_tail);
}
