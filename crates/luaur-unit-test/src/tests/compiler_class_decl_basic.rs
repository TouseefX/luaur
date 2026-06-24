#[cfg(test)]
#[test]
fn compiler_class_decl_basic() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let source = r#"
        class Point
            public x: number
            public y: number
        end
        print(Point)
    "#;

    let result = compile_function(source, 0, 0, 0);
    let expected = r#"
LOADKX R0 K3 [class Point (props: 2, methods: 0)]
GETGLOBAL R1 K4 ['print']
MOVE R2 R0
CALL R1 1 0
RETURN R0 0
"#;

    assert_eq!("\n".to_string() + &result, expected);
}
