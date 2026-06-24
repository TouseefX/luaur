#[cfg(test)]
#[test]
fn compiler_fold_const_table_props() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;
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

    let actual = compile_function(
        r#"local t = { hello = "world" }
return t.hello"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 2
LOADK R1 K1 ['world']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local t = { hello = "world" }
return t["hello"]"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 2
LOADK R1 K1 ['world']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1, green = 2, blue = 3}

return color.red, color["green"], color.blue"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 6
LOADN R1 1
LOADN R2 2
LOADN R3 3
RETURN R1 3
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1, green = 2, blue = 3}

return color.red + color.green + color.blue"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 6
LOADN R1 6
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1}
color.blue = 3
return color.red"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 2
LOADN R1 3
SETTABLEKS R1 R0 K3 ['blue']
GETTABLEKS R1 R0 K0 ['red']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1}
color["red"] = 3
return color.red"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 2
LOADN R1 3
SETTABLEKS R1 R0 K0 ['red']
GETTABLEKS R1 R0 K0 ['red']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1, blue = {}}
color["blue"]["red"] = 3
return color.red"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 3
NEWTABLE R1 0 0
SETTABLEKS R1 R0 K2 ['blue']
GETTABLEKS R1 R0 K2 ['blue']
LOADN R2 3
SETTABLEKS R2 R1 K0 ['red']
GETTABLEKS R1 R0 K0 ['red']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1}
color[color.red] = 3
return color.red"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 2
GETTABLEKS R1 R0 K0 ['red']
LOADN R2 3
SETTABLE R2 R0 R1
GETTABLEKS R1 R0 K0 ['red']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local function id(x) return x end
local color = {red = 1}
id(color)
return color.red"#,
        1,
        1,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['id']
DUPTABLE R1 3
MOVE R2 R0
MOVE R3 R1
CALL R2 1 0
GETTABLEKS R2 R1 K1 ['red']
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local function id(x) return x end
local color = {red = 1}
id(color.red)
return color.red"#,
        1,
        1,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['id']
DUPTABLE R1 3
MOVE R2 R0
LOADN R3 1
CALL R2 1 0
LOADN R2 1
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local function id(x) return x end
local t = { inner = { x = 1 } }
id(t.inner)
return t.inner.x"#,
        1,
        1,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['id']
DUPTABLE R1 2
DUPTABLE R2 5
SETTABLEKS R2 R1 K1 ['inner']
MOVE R2 R0
GETTABLEKS R3 R1 K1 ['inner']
CALL R2 1 0
GETTABLEKS R2 R1 K1 ['inner']
GETTABLEKS R2 R2 K3 ['x']
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local color = {red = 1}
color:test()
return color.red"#,
        0,
        1,
        0,
    );
    let expected = r#"
DUPTABLE R0 2
NAMECALL R1 R0 K3 ['test']
CALL R1 1 0
GETTABLEKS R1 R0 K0 ['red']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local function id(x) return x end
local t = { x = 1 }
local u = { [t] = true }
id(u)
return t.x"#,
        1,
        1,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['id']
DUPTABLE R1 3
NEWTABLE R2 1 0
LOADB R3 1
SETTABLE R3 R2 R1
MOVE R3 R0
MOVE R4 R2
CALL R3 1 0
GETTABLEKS R3 R1 K1 ['x']
RETURN R3 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local function id(x) return x end
local t = { x = 1 }
u[t] = 100
id(u)
return t.x"#,
        1,
        1,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['id']
DUPTABLE R1 3
GETIMPORT R2 5 [u]
LOADN R3 100
SETTABLE R3 R2 R1
MOVE R2 R0
GETIMPORT R3 5 [u]
CALL R2 1 0
GETTABLEKS R2 R1 K1 ['x']
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function(
        r#"local function id(x) return x end
local t = { x = 1 }
u[t] += 100
id(u)
return t.x"#,
        1,
        1,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['id']
DUPTABLE R1 3
GETIMPORT R2 5 [u]
GETTABLE R3 R2 R1
ADDK R3 R3 K6 [100]
SETTABLE R3 R2 R1
MOVE R2 R0
GETIMPORT R3 5 [u]
CALL R2 1 0
GETTABLEKS R2 R1 K1 ['x']
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function_0(
        r#"local t = {[""] = 1}
return t[""]"#,
    );
    let expected = r#"
NEWTABLE R0 1 0
LOADN R1 1
SETTABLEKS R1 R0 K0 ['']
GETTABLEKS R1 R0 K0 ['']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function_0(
        r#"local t = {a = 1, ["a"] = 2}
return t.a"#,
    );
    let expected = r#"
NEWTABLE R0 2 0
LOADN R1 1
SETTABLEKS R1 R0 K0 ['a']
LOADN R1 2
SETTABLEKS R1 R0 K0 ['a']
GETTABLEKS R1 R0 K0 ['a']
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    let actual = compile_function_0(
        r#"local t = {["a"] = 5, ["a\0"] = 2}
return t.a - t["a\0"]"#,
    );
    let expected = r#"
NEWTABLE R0 2 0
LOADN R1 5
SETTABLEKS R1 R0 K0 ['a']
LOADN R1 2
SETTABLEKS R1 R0 K1 ['a\x00']
LOADN R1 3
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());
}
