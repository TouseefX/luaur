#[cfg(test)]
#[test]
fn compiler_table_constant_string_index() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _luau_compile_duptable_constant_pack2 =
        ScopedFastFlag::new(&FFlag::LuauCompileDuptableConstantPack2, true);
    let _sff = ScopedFastFlag::new(&FFlag::LuauCompilePropagateTableProps2, true);

    let actual1 = "\n".to_string() + &compile_function_0("local t = { a = 2 }\nreturn t['a']");
    let expected1 = "\nDUPTABLE R0 2\nLOADN R1 2\nRETURN R1 1\n";
    assert_eq!(actual1, expected1);

    let actual2 = "\n".to_string() + &compile_function_0("local t = {}\nt['a'] = 2");
    let expected2 = "\nNEWTABLE R0 0 0\nLOADN R1 2\nSETTABLEKS R1 R0 K0 ['a']\nRETURN R0 0\n";
    assert_eq!(actual2, expected2);
}
