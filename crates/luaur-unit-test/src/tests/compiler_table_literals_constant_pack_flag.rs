#[cfg(test)]
#[test]
fn compiler_table_literals_constant_pack_flag() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _scoped_flag = ScopedFastFlag::new(&FFlag::LuauCompileDuptableConstantPack2, true);

    assert_eq!(
        "\n".to_string() + &compile_function_0("return {a=1,b=2,c=3}"),
        "\nDUPTABLE R0 6\nRETURN R0 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("return {a=1,b=2},{b=3,a=4},{a=5,b=6}"),
        "\nDUPTABLE R0 4\nDUPTABLE R1 7\nDUPTABLE R2 10\nRETURN R0 3\n"
    );
}
