#[cfg(test)]
#[test]
fn compiler_fold_const_table_props_return_upvalue() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _luau_compile_propagate_table_props =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompilePropagateTableProps2, true);
    let _luau_compile_duptable_constant_pack =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileDuptableConstantPack2, true);
    let _luau_compile_new_table_mutation_tracker = ScopedFastFlag::new(
        &luaur_common::FFlag::LuauCompileNewTableMutationTracker,
        true,
    );
    let _luau_compile_fold_optimize =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileFoldOptimize, true);

    let actual1 = compile_function(
        r#"local t = { x = 1 }
local function get() return t.x end
return t, get"#,
        0,
        0,
        0,
    );
    let expected1 = "\nGETUPVAL R0 0\nGETTABLEKS R0 R0 K0 ['x']\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function(
        r#"local function make()
    local t = { x = 1 }
    local function get() return t.x end
    return t, get
end
return make()"#,
        0,
        0,
        0,
    );
    let expected2 = "\nGETUPVAL R0 0\nGETTABLEKS R0 R0 K0 ['x']\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
