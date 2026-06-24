#[cfg(test)]
#[test]
fn compiler_nested_namecall() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag;

    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    let actual = compile_function_0(
        "local obj = ...\n\
         return obj:Method(1):Method(2):Method(3)",
    );

    let expected = "\nGETVARARGS R0 1\nLOADN R3 1\nNAMECALL R1 R0 K0 ['Method']\nCALL R1 2 1\nLOADN R3 2\nNAMECALL R1 R1 K0 ['Method']\nCALL R1 2 1\nLOADN R3 3\nNAMECALL R1 R1 K0 ['Method']\nCALL R1 2 -1\nRETURN R1 -1\n";

    assert_eq!(format!("\n{}", actual), expected);
}
