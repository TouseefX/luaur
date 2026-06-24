#[cfg(test)]
#[test]
fn compiler_fold_const_table_props_or_and() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let luau_compile_propagate_table_props =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompilePropagateTableProps2, true);
    let _luau_compile_duptable_constant_pack =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileDuptableConstantPack2, true);
    let _luau_compile_new_table_mutation_tracker = ScopedFastFlag::new(
        &luaur_common::FFlag::LuauCompileNewTableMutationTracker,
        true,
    );
    let _luau_compile_fold_optimize =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileFoldOptimize, true);

    // handle 'or'
    let result1 = compile_function_0("local t = { a = 1, b = 2 }\nreturn t.a or t.b\n");
    let expected1 = "\nDUPTABLE R0 4\nLOADN R1 1\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &result1, expected1);

    // handle 'and'
    let result2 = compile_function_0("local t = { a = 1, b = 2 }\nreturn t.a and t.b\n");
    let expected2 = "\nDUPTABLE R0 4\nLOADN R1 2\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &result2, expected2);

    // or with falsy left
    let result3 = compile_function_0("local t = { a = false, b = 42 }\nreturn t.a or t.b\n");
    let expected3 = "\nDUPTABLE R0 4\nLOADN R1 42\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &result3, expected3);

    // and with falsy left
    let result4 = compile_function_0("local t = { a = nil, b = 42 }\nreturn t.a and t.b\n");
    let expected4 = "\nDUPTABLE R0 4\nLOADNIL R1\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &result4, expected4);

    // nested
    let result5 =
        compile_function_0("local t = { a = nil, b = false, c = 99 }\nreturn t.a or t.b or t.c\n");
    let expected5 = "\nDUPTABLE R0 6\nLOADN R1 99\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &result5, expected5);
}
