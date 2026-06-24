#[cfg(test)]
#[test]
fn compiler_interp_string_const_fold() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _luau_compile_string_interp_temp_reg =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileStringInterpTargetTop, true);

    let result1 = compile_function_0(r#"local empty = ""; return `{empty}`"#);
    let expected1 = "\nLOADK R0 K0 ['']\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function_0(r#"local world = "world"; return `hello, {world}!`"#);
    let expected2 = "\nLOADK R0 K0 ['hello, world!']\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function_0(
        r#"local not_string = 42; local world = "world"; return `hello, {world} {not_string}!`"#,
    );
    let expected3 = "\nLOADK R0 K0 ['hello, world %*!']\nLOADN R2 42\nNAMECALL R0 R0 K1 ['format']\nCALL R0 2 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result3), expected3);

    let result4 = compile_function_0(
        r#"local not_string = 42; local str = "%s%s%s"; return `hello, {str} {not_string}!`"#,
    );
    let expected4 = "\nLOADK R0 K0 ['hello, %%s%%s%%s %*!']\nLOADN R2 42\nNAMECALL R0 R0 K1 ['format']\nCALL R0 2 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result4), expected4);
}
