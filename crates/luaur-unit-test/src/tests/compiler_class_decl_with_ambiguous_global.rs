#[cfg(test)]
#[test]
fn compiler_class_decl_with_ambiguous_global() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _sffs = [
        ScopedFastFlag::new(&FFlag::LuauCompileStringInterpTargetTop, true),
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true),
        ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true),
    ];

    let source = r#"
        class Point
            public x: number
            public y: number
            function print(self)
                print(`Point(x = {self.x}, y = {self.y})`)
            end
        end
        return { Point = Point }
    "#;

    let res0 = "\n".to_string() + &compile_function(source, 0, 0, 0);
    let expected0 = r#"
GETGLOBAL R1 K0 ['print']
LOADK R2 K1 ['Point(x = %*, y = %*)']
GETTABLEKS R4 R0 K2 ['x']
GETTABLEKS R5 R0 K3 ['y']
NAMECALL R2 R2 K4 ['format']
CALL R2 3 1
CALLFB R1 1 0 [0]
RETURN R0 0
"#;
    assert_eq!(format!("\n{}", res0), format!("\n{}", expected0));

    let res1 = "\n".to_string() + &compile_function(source, 1, 0, 0);
    let expected1 = r#"
LOADKX R0 K4 [class Point (props: 2, methods: 1)]
NEWCLOSURE R1 P0
NEWCLASSMEMBER R0 R1 ['print']
DUPTABLE R1 5
LOADK R2 K0 ['Point']
SETTABLE R0 R1 R2
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", res1), format!("\n{}", expected1));
}
