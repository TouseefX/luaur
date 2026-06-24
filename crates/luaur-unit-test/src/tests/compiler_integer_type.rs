#[cfg(test)]
#[test]
fn compiler_integer_type() {
    use crate::functions::compile_function_0::compile_function_0;
    use luaur_common::FFlag::LuauIntegerType2;

    if !LuauIntegerType2.get() {
        return;
    }

    // i suffix
    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = 123i\nreturn a"),
        "\nLOADK R0 K0 [123]\nRETURN R0 1\n"
    );

    // separators
    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = 1_000_000i\nreturn a"),
        "\nLOADK R0 K0 [1000000]\nRETURN R0 1\n"
    );

    // hex
    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = 0xABABi\nreturn a"),
        "\nLOADK R0 K0 [43947]\nRETURN R0 1\n"
    );

    // binary
    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = 0b100101i\nreturn a"),
        "\nLOADK R0 K0 [37]\nRETURN R0 1\n"
    );

    // Has to be exactly representable; overflow is a parse error
    let source1 = "local a = 9999999999999999999999999i";
    let source2 = "local a = 2.37i";

    use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
    struct NoEncoder;
    impl BytecodeEncoder for NoEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
    }
    let no_encoder: *mut dyn BytecodeEncoder =
        core::ptr::null_mut::<NoEncoder>() as *mut dyn BytecodeEncoder;

    let bc1 = luaur_compiler::functions::compile::compile(
        &source1.to_string(),
        &luaur_compiler::records::compile_options::CompileOptions::default(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
        no_encoder,
    );
    let bc2 = luaur_compiler::functions::compile::compile(
        &source2.to_string(),
        &luaur_compiler::records::compile_options::CompileOptions::default(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
        no_encoder,
    );

    // 0 acts as a special marker for error bytecode
    assert_eq!(bc1.as_bytes()[0], 0);
    assert_eq!(bc2.as_bytes()[0], 0);
}
