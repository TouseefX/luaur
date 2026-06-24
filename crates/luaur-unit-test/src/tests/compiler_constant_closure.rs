#[cfg(test)]
#[test]
fn compiler_constant_closure() {
    use crate::functions::compile_function::compile_function;

    // closures without upvalues are created when bytecode is loaded
    let actual = compile_function("return function() end", 1, 1, 0);
    let expected = "\nDUPCLOSURE R0 K0 []\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // they can access globals just fine
    let actual = compile_function("return function() print(\"hi\") end", 1, 1, 0);
    let expected = "\nDUPCLOSURE R0 K0 []\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // if they need upvalues, we can't create them before running the code (but see SharedClosure test)
    let actual = compile_function(
        "function test()\n    local print = print\n    return function() print(\"hi\") end\nend",
        1,
        1,
        0,
    );
    let expected = "\nGETIMPORT R0 1 [print]\nNEWCLOSURE R1 P0\nCAPTURE VAL R0\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // if they don't need upvalues but we sense that environment may be modified, we disable this to avoid fenv-related identity confusion
    let actual = compile_function(
        "setfenv(1, {})\nreturn function() print(\"hi\") end",
        1,
        1,
        0,
    );
    let expected = "\nGETIMPORT R0 1 [setfenv]\nLOADN R1 1\nNEWTABLE R2 0 0\nCALL R0 2 0\nNEWCLOSURE R0 P0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // note that fenv analysis isn't flow-sensitive right now, which is sort of a feature
    let actual = compile_function(
        "if false then setfenv(1, {}) end\nreturn function() print(\"hi\") end",
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R0 P0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());
}
