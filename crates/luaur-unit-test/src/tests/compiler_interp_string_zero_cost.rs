#[cfg(test)]
#[test]
fn compiler_interp_string_zero_cost() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _luau_compile_string_interp_temp_reg =
        ScopedFastFlag::new(&FFlag::LuauCompileStringInterpTargetTop, true);

    let result = compile_function_0("local _ = `hello, {42}!`");
    let expected = "\nLOADK R0 K0 ['hello, %*!']\nLOADN R2 42\nNAMECALL R0 R0 K1 ['format']\nCALL R0 2 1\nRETURN R0 0\n";

    assert_eq!("\n".to_string() + &result, expected);
}
