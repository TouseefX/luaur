#[cfg(test)]
#[test]
fn compiler_loop_continue_until() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    // it's valid to use locals defined inside the loop in until expression if they're defined before continue
    let actual = compile_function_0(
        "repeat local r = math.random() if r > 0.5 then continue end r = r + 0.3 until r < 0.5",
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFLT R1 R0 L1\nADDK R0 R0 K4 [0.29999999999999999]\nL1: LOADK R1 K3 [0.5]\nJUMPIFLT R0 R1 L2\nJUMPBACK L0\nL2: RETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // it's however invalid to use locals if they are defined after continue
    let bytecode = &mut luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None);
    let source = r#"
repeat
    local r = math.random()
    if r > 0.5 then
        continue
    end
    local rr = r + 0.3
until rr < 0.5
"#
    .to_string();
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options(
            bytecode,
            &source,
            &options,
            &parse_options,
        );
    }));
    assert!(result.is_err(), "Expected CompileError");

    // but it's okay if continue is inside a non-repeat..until loop, or inside a loop that doesn't use the local (here `continue` just terminates
    // inner loop)
    let actual = compile_function_0(
        "repeat local r = math.random() repeat if r > 0.5 then continue end r = r - 0.1 until true r = r + 0.3 until r < 0.5",
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFLT R1 R0 L1\nSUBK R0 R0 K4 [0.10000000000000001]\nL1: ADDK R0 R0 K5 [0.29999999999999999]\nLOADK R1 K3 [0.5]\nJUMPIFLT R0 R1 L2\nJUMPBACK L0\nL2: RETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // and it's also okay to use a local defined in the until expression as long as it's inside a function!
    let actual = compile_function(
        "repeat local r = math.random() if r > 0.5 then continue end r = r + 0.3 until (function() local a = r return a < 0.5 end)()",
        1,
        1,
        0,
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFLT R1 R0 L1\nADDK R0 R0 K4 [0.29999999999999999]\nL1: NEWCLOSURE R1 P0\nCAPTURE REF R0\nCALL R1 0 1\nJUMPIF R1 L2\nCLOSEUPVALS R0\nJUMPBACK L0\nL2: CLOSEUPVALS R0\nRETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // but not if the function just refers to an upvalue
    let bytecode = &mut luaur_bytecode::records::bytecode_builder::BytecodeBuilder::new(None);
    let source = r#"
repeat
    local r = math.random()
    if r > 0.5 then
        continue
    end
    local rr = r + 0.3
until (function() return rr end)() < 0.5
"#
    .to_string();
    let options = luaur_compiler::records::compile_options::CompileOptions::default();
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options(
            bytecode,
            &source,
            &options,
            &parse_options,
        );
    }));
    assert!(result.is_err(), "Expected CompileError");

    // unless that upvalue is from an outer scope
    let actual = compile_function_0(
        "local stop = false stop = true function test() repeat local r = math.random() if r > 0.5 then continue end r = r + 0.3 until stop or r < 0.5 end",
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALLFB R0 0 1 [0]\nLOADK R1 K3 [0.5]\nJUMPIFLT R1 R0 L1\nADDK R0 R0 K4 [0.29999999999999999]\nL1: GETUPVAL R1 0\nJUMPIF R1 L2\nLOADK R1 K3 [0.5]\nJUMPIFLT R0 R1 L2\nJUMPBACK L0\nL2: RETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // including upvalue references from a function expression
    let actual = compile_function(
        "local stop = false stop = true function test() repeat local r = math.random() if r > 0.5 then continue end r = r + 0.3 until (function() return stop or r < 0.5 end)() end",
        1,
        1,
        0,
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALLFB R0 0 1 [0]\nLOADK R1 K3 [0.5]\nJUMPIFLT R1 R0 L1\nADDK R0 R0 K4 [0.29999999999999999]\nL1: NEWCLOSURE R1 P0\nCAPTURE UPVAL U0\nCAPTURE REF R0\nCALLFB R1 0 1 [1]\nJUMPIF R1 L2\nCLOSEUPVALS R0\nJUMPBACK L0\nL2: CLOSEUPVALS R0\nRETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());
}
