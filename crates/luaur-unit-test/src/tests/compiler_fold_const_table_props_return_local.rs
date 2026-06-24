#[cfg(test)]
#[test]
fn compiler_fold_const_table_props_return_local() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);
    let _luau_compile_propagate_table_props =
        ScopedFastFlag::new(&FFlag::LuauCompilePropagateTableProps2, true);
    let _luau_compile_duptable_constant_pack =
        ScopedFastFlag::new(&FFlag::LuauCompileDuptableConstantPack2, true);
    let _luau_compile_new_table_mutation_tracker =
        ScopedFastFlag::new(&FFlag::LuauCompileNewTableMutationTracker, true);
    let _luau_compile_fold_optimize = ScopedFastFlag::new(&FFlag::LuauCompileFoldOptimize, true);

    let actual1 = compile_function_0(
        r#"local t = { a = 1, b = 2 }
print(t.a + t.b)
return t
"#,
    );
    let expected1 = "\nDUPTABLE R0 4\nGETIMPORT R1 6 [print]\nGETTABLEKS R3 R0 K0 ['a']\nGETTABLEKS R4 R0 K2 ['b']\nADD R2 R3 R4\nCALL R1 1 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0(
        r#"local function foo()
    local t = { a = 1, b = 2 }
    print(t.a + t.b)
    return t
end
return foo()
"#,
    );
    let expected2 = "\nDUPTABLE R0 4\nGETIMPORT R1 6 [print]\nGETTABLEKS R3 R0 K0 ['a']\nGETTABLEKS R4 R0 K2 ['b']\nADD R2 R3 R4\nCALLFB R1 1 0 [0]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
