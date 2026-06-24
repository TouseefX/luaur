#[cfg(test)]
#[test]
fn compiler_class_decl_with_method() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _ff = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let source = r#"
        class Point
            public x: number
            public y: number
            function magnitude(self)
                return self.x * self.x + self.y * self.y
            end
        end
        print(Point)
    "#;

    let res0 = "\n".to_string() + &compile_function(source, 0, 0, 0);
    let expected0 = r#"
GETTABLEKS R3 R0 K0 ['x']
GETTABLEKS R4 R0 K0 ['x']
MUL R2 R3 R4
GETTABLEKS R4 R0 K1 ['y']
GETTABLEKS R5 R0 K1 ['y']
MUL R3 R4 R5
ADD R1 R2 R3
RETURN R1 1
"#;
    assert_eq!(res0, expected0);

    let res1 = "\n".to_string() + &compile_function(source, 1, 0, 0);
    let expected1 = r#"
LOADKX R0 K4 [class Point (props: 2, methods: 1)]
NEWCLOSURE R1 P0
NEWCLASSMEMBER R0 R1 ['magnitude']
GETGLOBAL R1 K5 ['print']
MOVE R2 R0
CALL R1 1 0
RETURN R0 0
"#;
    assert_eq!(res1, expected1);
}
