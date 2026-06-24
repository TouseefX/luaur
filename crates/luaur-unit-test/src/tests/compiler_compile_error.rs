use crate::functions::rep::rep;

#[cfg(test)]
#[test]
fn compiler_compile_error() {
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

    let options = CompileOptions::default();
    let parse_options = ParseOptions::default();

    let source = format!("local {}", rep("a,", 300)) + "a = ...";

    // fails to parse
    let bc1 = compile(
        &(source.clone() + " !#*$!#$^&!*#&$^*"),
        &options,
        &parse_options,
        no_encoder,
    );

    // parses, but fails to compile (too many locals)
    let bc2 = compile(&source, &options, &parse_options, no_encoder);

    // 0 acts as a special marker for error bytecode
    assert_eq!(bc1.as_bytes()[0], 0);
    assert_eq!(bc2.as_bytes()[0], 0);
}
