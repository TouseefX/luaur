#[cfg(test)]
#[test]
fn compiler_capture_immutable() {
    use crate::functions::compile_function::compile_function;

    // capture argument: note capture by value
    let actual = compile_function(
        "function foo(a, b) return function() return a end end",
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R2 P0\nCAPTURE VAL R0\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // capture mutable argument: note capture by reference + close
    let actual = compile_function(
        "function foo(a, b) a = 1 return function() return a end end",
        1,
        1,
        0,
    );
    let expected = "\nLOADN R0 1\nNEWCLOSURE R2 P0\nCAPTURE REF R0\nCLOSEUPVALS R0\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // capture two arguments, one mutable, one immutable
    let actual = compile_function(
        "function foo(a, b) a = 1 return function() return a + b end end",
        1,
        1,
        0,
    );
    let expected = "\nLOADN R0 1\nNEWCLOSURE R2 P0\nCAPTURE REF R0\nCAPTURE VAL R1\nCLOSEUPVALS R0\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // capture self
    let actual = compile_function(
        "function bar:foo(a, b) return function() return self end end",
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R3 P0\nCAPTURE VAL R0\nRETURN R3 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // capture mutable self (who mutates self?!?)
    let actual = compile_function(
        "function bar:foo(a, b) self = 42 return function() return self end end",
        1,
        1,
        0,
    );
    let expected = "\nLOADN R0 42\nNEWCLOSURE R3 P0\nCAPTURE REF R0\nCLOSEUPVALS R0\nRETURN R3 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // capture upvalue: one mutable, one immutable
    let actual = compile_function(
        "local a, b = math.rand() a = 42 function foo() return function() return a + b end end",
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R0 P0\nCAPTURE UPVAL U0\nCAPTURE UPVAL U1\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // recursive capture
    let actual = compile_function("local function foo() return foo() end", 1, 1, 0);
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nCAPTURE VAL R0\nRETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // multi-level recursive capture
    let actual = compile_function(
        "local function foo() return function() return foo() end end",
        1,
        1,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 []\nCAPTURE UPVAL U0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // multi-level recursive capture where function isn't top-level
    let actual = compile_function(
        "local function foo()\n    local function bar()\n        return function() return bar() end\n    end\nend",
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R0 P0\nCAPTURE UPVAL U0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // capture mutated table
    let actual = compile_function(
        "local function foo()\n    local t = {}\n    t[1] = 42\n    return function() return t end\nend",
        1,
        1,
        0,
    );
    let expected = "\nNEWTABLE R0 0 1\nLOADN R1 42\nSETTABLEN R1 R0 1\nNEWCLOSURE R1 P0\nCAPTURE VAL R0\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());
}
