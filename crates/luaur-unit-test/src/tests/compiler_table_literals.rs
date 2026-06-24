#[cfg(test)]
#[test]
fn compiler_table_literals() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    let LuauCompileDuptableConstantPack2 =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileDuptableConstantPack2, true);
    let actual = compile_function_0("return {}");
    let expected = "\nNEWTABLE R0 0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("local a a = {a} return a");
    let expected =
        "\nLOADNIL R0\nNEWTABLE R1 0 1\nMOVE R2 R0\nSETLIST R1 R2 1 [1]\nMOVE R0 R1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {1,2,3}");
    let expected =
        "\nNEWTABLE R0 0 3\nLOADN R1 1\nLOADN R2 2\nLOADN R3 3\nSETLIST R0 R1 3 [1]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17}");
    let expected = "\nNEWTABLE R0 0 17\nLOADN R1 1\nLOADN R2 2\nLOADN R3 3\nLOADN R4 4\nLOADN R5 5\nLOADN R6 6\nLOADN R7 7\nLOADN R8 8\nLOADN R9 9\nLOADN R10 10\nLOADN R11 11\nLOADN R12 12\nLOADN R13 13\nLOADN R14 14\nLOADN R15 15\nLOADN R16 16\nSETLIST R0 R1 16 [1]\nLOADN R1 17\nSETLIST R0 R1 1 [17]\nRETURN R0 1\n" ;
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {...}");
    let expected = "\nNEWTABLE R0 0 0\nGETVARARGS R1 -1\nSETLIST R0 R1 -1 [1]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {1,2,3,...}");
    let expected = "\nNEWTABLE R0 0 3\nLOADN R1 1\nLOADN R2 2\nLOADN R3 3\nGETVARARGS R4 -1\nSETLIST R0 R1 -1 [1]\nRETURN R0 1\n" ;
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {a=1,b=2,c=3}");
    let expected = "\nDUPTABLE R0 6\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {a=1,b=2,3,4}");
    let expected = "\nNEWTABLE R0 2 2\nLOADN R3 1\nSETTABLEKS R3 R0 K0 ['a']\nLOADN R3 2\nSETTABLEKS R3 R0 K1 ['b']\nLOADN R1 3\nLOADN R2 4\nSETLIST R0 R1 2 [1]\nRETURN R0 1\n" ;
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("a = 7 return {[a]=42}");
    let expected = "\nLOADN R0 7\nSETGLOBAL R0 K0 ['a']\nNEWTABLE R0 1 0\nGETGLOBAL R1 K0 ['a']\nLOADN R2 42\nSETTABLE R2 R0 R1\nRETURN R0 1\n" ;
    assert_eq!(format!("\n{}", actual), expected);
    let actual = compile_function_0("return {a=1,b=2},{b=3,a=4},{a=5,b=6}");
    let expected = "\nDUPTABLE R0 4\nDUPTABLE R1 7\nDUPTABLE R2 10\nRETURN R0 3\n";
    assert_eq!(format!("\n{}", actual), expected);
}
