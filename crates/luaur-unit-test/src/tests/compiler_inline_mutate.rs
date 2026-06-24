#[cfg(test)]
#[test]
fn compiler_inline_mutate() {
    use crate::functions::compile_function::compile_function;

    // if the argument is mutated, it gets a register even if the value is constant
    let actual = compile_function(
        "local function foo(a)\n    a = a or 5\n    return a\nend\n\nlocal x = foo(42)\nreturn x\n",
        1,
        2,
        0,
    );
    let expected =
        "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R2 42\nORK R2 R2 K1 [5]\nMOVE R1 R2\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // if the argument is a local, it can be used directly
    let actual = compile_function(
        "local function foo(a)\n    return a\nend\n\nlocal x = ...\nlocal y = foo(x)\nreturn y\n",
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 1\nMOVE R2 R1\nRETURN R2 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // ... but if it's mutated, we move it in case it is mutated through a capture during the inlined function
    let actual = compile_function(
        "local function foo(a)\n    return a\nend\n\nlocal x = ...\nx = nil\nlocal y = foo(x)\nreturn y\n",
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 1\nLOADNIL R1\nMOVE R3 R1\nMOVE R2 R3\nRETURN R2 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // we also don't inline functions if they have been assigned to
    let actual = compile_function(
        "local function foo(a)\n    return a\nend\n\nfoo = foo\n\nlocal x = foo(42)\nreturn x\n",
        1,
        2,
        0,
    );
    let expected =
        "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nLOADN R2 42\nCALL R1 1 1\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual), expected);
}
