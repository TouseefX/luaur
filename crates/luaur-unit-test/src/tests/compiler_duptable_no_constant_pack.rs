#[cfg(test)]
#[test]
fn compiler_duptable_no_constant_pack() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauCompileDuptableConstantPack2;

    let _flag = ScopedFastFlag::new(&LuauCompileDuptableConstantPack2, true);

    let actual = compile_function(
        r#"local t = { a = 2, a = function() end, a = 3 }
return t['a']
"#,
        1,
        1,
        0,
    );

    let expected = "\nDUPTABLE R0 3\nLOADN R1 2\nSETTABLEKS R1 R0 K0 ['a']\nDUPCLOSURE R1 K4 ['a']\nSETTABLEKS R1 R0 K0 ['a']\nLOADN R1 3\nSETTABLEKS R1 R0 K0 ['a']\nGETTABLEKS R1 R0 K0 ['a']\nRETURN R1 1\n";

    assert_eq!(format!("\n{}", actual), expected);
}
