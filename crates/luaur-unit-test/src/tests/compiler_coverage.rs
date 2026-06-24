#[cfg(test)]
#[test]
fn compiler_coverage() {
    use crate::functions::compile_function_0_coverage::compile_function_0_coverage;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _luau_compile_duptable_constant_pack2 =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileDuptableConstantPack2, true);

    let actual1 = compile_function_0_coverage(
        r#"
print(1)
print(2)
"#,
        1,
    );
    let expected1 = "\n2: COVERAGE\n2: GETIMPORT R0 1 [print]\n2: LOADN R1 1\n2: CALL R0 1 0\n3: COVERAGE\n3: GETIMPORT R0 1 [print]\n3: LOADN R1 2\n3: CALL R0 1 0\n4: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0_coverage(
        r#"
if x then
    print(1)
else
    print(2)
end
"#,
        1,
    );
    let expected2 = "\n2: COVERAGE\n2: GETIMPORT R0 1 [x]\n2: JUMPIFNOT R0 L0\n3: COVERAGE\n3: GETIMPORT R0 3 [print]\n3: LOADN R1 1\n3: CALL R0 1 0\n3: RETURN R0 0\n5: L0: COVERAGE\n5: GETIMPORT R0 3 [print]\n5: LOADN R1 2\n5: CALL R0 1 0\n7: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function_0_coverage(
        r#"
if x then
    -- first
    print(1)
else
    -- second
    print(2)
end
"#,
        1,
    );
    let expected3 = "\n2: COVERAGE\n2: GETIMPORT R0 1 [x]\n2: JUMPIFNOT R0 L0\n4: COVERAGE\n4: GETIMPORT R0 3 [print]\n4: LOADN R1 1\n4: CALL R0 1 0\n4: RETURN R0 0\n7: L0: COVERAGE\n7: GETIMPORT R0 3 [print]\n7: LOADN R1 2\n7: CALL R0 1 0\n9: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    let actual4 = compile_function_0_coverage(
        r#"
local c = ...
local t = {
    a = 1,
    b = 2,
    c = c
}
"#,
        2,
    );
    let expected4 = "\n2: COVERAGE\n2: COVERAGE\n2: GETVARARGS R0 1\n3: COVERAGE\n3: COVERAGE\n3: DUPTABLE R1 5\n6: COVERAGE\n6: SETTABLEKS R0 R1 K4 ['c']\n8: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual4), expected4);
}
