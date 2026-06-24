#[cfg(test)]
#[test]
fn compiler_side_effects() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;

    let actual1 = compile_function_0(
        "local x = 5, print\n\
         local y = 5, 42\n\
         local z = 5, table.find -- considered side effecting because of metamethods",
    );
    let expected1 =
        "\nLOADN R0 5\nLOADN R1 5\nLOADN R2 5\nGETIMPORT R3 2 [table.find]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function(
        "local function test1()\n    return 42\nend\n\n\
         local function test2()\n    return print\nend\n\n\
         local function test3()\n    return function() print(test3) end\nend\n\n\
         local function test4()\n    return table.find -- considered side effecting because of metamethods\nend\n\n\
         test1()\ntest2()\ntest3()\ntest4()",
        5,
        2,
        0,
    );
    let expected2 = "\nDUPCLOSURE R0 K0 ['test1']\nDUPCLOSURE R1 K1 ['test2']\nDUPCLOSURE R2 K2 ['test3']\nCAPTURE VAL R2\n\
         DUPCLOSURE R3 K3 ['test4']\nGETIMPORT R4 6 [table.find]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
