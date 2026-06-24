//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:11904:compiler_export_local_bytecode`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> function compileFunction0 (tests/Compiler.test.cpp)
//!   - translates_to -> rust_item compiler_export_local_bytecode

#[cfg(test)]
#[test]
fn compiler_export_local_bytecode() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _sffs = [
        ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
        ScopedFastFlag::new(&FFlag::LuauConst2, true),
    ];

    // basic exported local: value is stored into the export table, then table is frozen and returned
    let actual = compile_function_0("export local x = 5");
    let expected = r#"
LOADN R0 5
NEWTABLE R1 0 0
SETTABLEKS R0 R1 K0 ['x']
GETIMPORT R2 3 [table.freeze]
MOVE R3 R1
CALL R2 1 1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // multiple exported locals are all stored into the same export table
    let actual = compile_function_0("export local x = 5\nexport local y = 10");
    let expected = r#"
LOADN R0 5
NEWTABLE R1 0 0
SETTABLEKS R0 R1 K0 ['x']
LOADN R2 10
SETTABLEKS R2 R1 K1 ['y']
GETIMPORT R3 4 [table.freeze]
MOVE R4 R1
CALL R3 1 1
RETURN R3 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // reassigning an exported local updates the export table
    let actual = compile_function_0("export local x = 5\nx = 10");
    let expected = r#"
LOADN R0 5
NEWTABLE R1 0 0
SETTABLEKS R0 R1 K0 ['x']
LOADN R2 10
SETTABLEKS R2 R1 K0 ['x']
GETIMPORT R2 3 [table.freeze]
MOVE R3 R1
CALL R2 1 1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
