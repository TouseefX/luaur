#[cfg(test)]
#[test]
fn compiler_inline_table_function() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _luau_compile_propagate_table_props =
        ScopedFastFlag::new(&FFlag::LuauCompilePropagateTableProps2, true);
    let _luau_compile_new_table_mutation_tracker =
        ScopedFastFlag::new(&FFlag::LuauCompileNewTableMutationTracker, true);
    let _luau_compile_fold_optimize = ScopedFastFlag::new(&FFlag::LuauCompileFoldOptimize, true);
    let _luau_compile_inline_table_functions =
        ScopedFastFlag::new(&FFlag::LuauCompileInlineTableFunctions, true);

    let result1 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end
}
return t.f(100)
"#,
        1,
        2,
        2,
    );
    let expected1 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nLOADN R1 101\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end
} :: any
return t.f(100)
"#,
        1,
        2,
        2,
    );
    let expected2 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nLOADN R1 101\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end
}
local g = t.f
return g(100)
"#,
        1,
        2,
        2,
    );
    let expected3 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nGETTABLEKS R1 R0 K0 ['f']\nLOADN R2 101\nRETURN R2 1\n";
    assert_eq!(format!("\n{}", result3), expected3);

    let result4 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end
}
return (t).f(100)
"#,
        1,
        2,
        2,
    );
    let expected4 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nLOADN R1 101\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result4), expected4);

    let result5 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end
}
return t.f<<number>>(100)
"#,
        1,
        2,
        2,
    );
    let expected5 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nLOADN R1 101\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result5), expected5);

    let result6 = compile_function(
        r#"local function id(x) return x end
local t = {
    f = function(x) return x + 1 end
}
id(t)
return t.f(1)
"#,
        2,
        2,
        2,
    );
    let expected6 = "\nDUPCLOSURE R0 K0 ['id']\nDUPTABLE R1 2\nDUPCLOSURE R2 K3 ['f']\nSETTABLEKS R2 R1 K1 ['f']\nGETTABLEKS R2 R1 K1 ['f']\nLOADN R3 1\nCALL R2 1 -1\nRETURN R2 -1\n";
    assert_eq!(format!("\n{}", result6), expected6);

    let result7 = compile_function(
        r#"local t = { f = function(x) return x + 1 end }
t.g = print
return t.f(1)
"#,
        1,
        2,
        2,
    );
    let expected7 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nGETIMPORT R1 4 [print]\nSETTABLEKS R1 R0 K5 ['g']\nGETTABLEKS R1 R0 K0 ['f']\nLOADN R2 1\nCALL R1 1 -1\nRETURN R1 -1\n";
    assert_eq!(format!("\n{}", result7), expected7);

    let result8 = compile_function(
        r#"local t = {
    [""] = "anything",
    f = function(x) return x + 1 end
}
return t.f(100)
"#,
        1,
        2,
        2,
    );
    let expected8 = "\nNEWTABLE R0 2 0\nLOADK R1 K0 ['anything']\nSETTABLEKS R1 R0 K1 ['']\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K3 ['f']\nLOADN R1 101\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result8), expected8);

    let result9 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end,
    ["f"] = function() return 2 end
}
return t.f(100)
"#,
        2,
        2,
        2,
    );
    let expected9 = "\nNEWTABLE R0 2 0\nDUPCLOSURE R1 K0 ['f']\nSETTABLEKS R1 R0 K1 ['f']\nDUPCLOSURE R1 K2 []\nSETTABLEKS R1 R0 K1 ['f']\nLOADN R1 2\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result9), expected9);

    let result10 = compile_function(
        r#"local t = {
    f = function(x) return x + 1 end,
    f = function() return 2 end
}
return t.f(100)
"#,
        2,
        2,
        2,
    );
    let expected10 = "\nDUPTABLE R0 1\nDUPCLOSURE R1 K2 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nDUPCLOSURE R1 K3 ['f']\nSETTABLEKS R1 R0 K0 ['f']\nLOADN R1 2\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result10), expected10);

    let result11 = compile_function(
        r#"local k = "f"
local t = {
    f = function(x) return x + 1 end,
    [k] = function() return 2 end
}
return t.f(100)
"#,
        2,
        2,
        2,
    );
    let expected11 = "\nNEWTABLE R0 2 0\nDUPCLOSURE R1 K0 ['f']\nSETTABLEKS R1 R0 K1 ['f']\nDUPCLOSURE R1 K2 []\nSETTABLEKS R1 R0 K1 ['f']\nLOADN R1 2\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", result11), expected11);

    let result12 = compile_function(
        r#"local k = ...
local t = {
    f = function(x) return x + 1 end,
    [k] = function() return 2 end
}
return t.f(100)
"#,
        2,
        2,
        2,
    );
    let expected12 = "\nGETVARARGS R0 1\nNEWTABLE R1 2 0\nDUPCLOSURE R2 K0 ['f']\nSETTABLEKS R2 R1 K1 ['f']\nDUPCLOSURE R2 K2 []\nSETTABLE R2 R1 R0\nGETTABLEKS R2 R1 K1 ['f']\nLOADN R3 100\nCALL R2 1 -1\nRETURN R2 -1\n";
    assert_eq!(format!("\n{}", result12), expected12);

    let result13 = compile_function(
        r#"local k = ...
local t = {
    [k] = function() return 2 end,
    f = function(x) return x + 1 end
}
return t.f(100)
"#,
        2,
        2,
        2,
    );
    let expected13 = "\nGETVARARGS R0 1\nNEWTABLE R1 2 0\nDUPCLOSURE R2 K0 []\nSETTABLE R2 R1 R0\nDUPCLOSURE R2 K1 ['f']\nSETTABLEKS R2 R1 K2 ['f']\nLOADN R2 101\nRETURN R2 1\n";
    assert_eq!(format!("\n{}", result13), expected13);
}
