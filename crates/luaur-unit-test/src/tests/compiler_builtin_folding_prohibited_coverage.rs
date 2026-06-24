//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Compiler.test.cpp:9423:compiler_builtin_folding_prohibited_coverage`
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
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> function lrotate (CodeGen/src/BitUtils.h)
//!   - calls -> function rrotate (CodeGen/src/BitUtils.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function compileFunction (tests/Compiler.test.cpp)
//!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
//!   - translates_to -> rust_item compiler_builtin_folding_prohibited_coverage

#[cfg(test)]
#[test]
fn compiler_builtin_folding_prohibited_coverage() {
    use crate::functions::compile_function::compile_function;
    use alloc::string::String;

    let builtins: [&str; 43] = [
        "math.abs",
        "math.acos",
        "math.asin",
        "math.atan2",
        "math.atan",
        "math.ceil",
        "math.cosh",
        "math.cos",
        "math.deg",
        "math.exp",
        "math.floor",
        "math.fmod",
        "math.ldexp",
        "math.log10",
        "math.log",
        "math.max",
        "math.min",
        "math.pow",
        "math.rad",
        "math.sinh",
        "math.sin",
        "math.sqrt",
        "math.tanh",
        "math.tan",
        "bit32.arshift",
        "bit32.band",
        "bit32.bnot",
        "bit32.bor",
        "bit32.bxor",
        "bit32.btest",
        "bit32.extract",
        "bit32.lrotate",
        "bit32.lshift",
        "bit32.replace",
        "bit32.rrotate",
        "bit32.rshift",
        "type",
        "string.byte",
        "string.len",
        "typeof",
        "math.clamp",
        "math.sign",
        "math.round",
    ];

    for func in builtins.iter() {
        let mut source = String::from("return ");
        source.push_str(func);
        source.push_str("()");

        let bc = compile_function(&source, 0, 2, 0);

        assert!(
            bc.contains("FASTCALL"),
            "expected FASTCALL for builtin {}",
            func
        );
    }
}
