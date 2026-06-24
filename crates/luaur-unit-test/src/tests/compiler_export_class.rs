//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:11976:compiler_export_class`
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
//!   - calls -> function compileFunction (tests/Compiler.test.cpp)
//!   - translates_to -> rust_item compiler_export_class

#[cfg(test)]
#[test]
fn compiler_export_class() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _sffs = [
        ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true),
        ScopedFastFlag::new(&FFlag::LuauConst2, true),
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true),
    ];

    let actual = compile_function_0(
        r#"
export class Point
    public x: number
    public y: number
end
"#,
    );
    let expected = r#"
LOADKX R0 K3 [class Point (props: 2, methods: 0)]
NEWTABLE R1 1 0
SETTABLEKS R0 R1 K0 ['Point']
GETIMPORT R2 6 [table.freeze]
MOVE R3 R1
CALL R2 1 1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function(
        r#"
export class Point
    public x: number
    public y: number

    function getX(self)
        return self.x
    end

    function getY(self)
        return self.y
    end
end
"#,
        2,
        1,
        0,
    );
    let expected = r#"
LOADKX R0 K7 [class Point (props: 2, methods: 2)]
DUPCLOSURE R1 K3 ['getX']
NEWCLASSMEMBER R0 R1 ['getX']
DUPCLOSURE R1 K5 ['getY']
NEWCLASSMEMBER R0 R1 ['getY']
NEWTABLE R1 1 0
SETTABLEKS R0 R1 K0 ['Point']
GETIMPORT R2 10 [table.freeze]
MOVE R3 R1
CALL R2 1 1
RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
