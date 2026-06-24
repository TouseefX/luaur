#[cfg(test)]
#[test]
fn compiler_compile_bytecode() {
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_bytecode::records::bytecode_encoder::BytecodeEncoder;
    use luaur_compiler::functions::compile::compile;
    use luaur_compiler::records::compile_options::CompileOptions;

    // Concrete placeholder so we can form a typed null `*mut dyn BytecodeEncoder`.
    struct NoEncoder;
    impl BytecodeEncoder for NoEncoder {
        fn encode(&mut self, _data: &mut [u32]) {}
    }
    let no_encoder: *mut dyn BytecodeEncoder =
        core::ptr::null_mut::<NoEncoder>() as *mut dyn BytecodeEncoder;

    // This is a coverage test, it just exercises bytecode dumping for correct and malformed code
    let options = CompileOptions::default();
    let parse_options = ParseOptions::default();

    let _ = compile(
        &"return 5".to_string(),
        &options,
        &parse_options,
        no_encoder,
    );
    let _ = compile(
        &"this is not valid lua, right?".to_string(),
        &options,
        &parse_options,
        no_encoder,
    );
}
