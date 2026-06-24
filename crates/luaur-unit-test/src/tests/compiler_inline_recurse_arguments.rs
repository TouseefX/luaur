#[cfg(test)]
#[test]
fn compiler_inline_recurse_arguments() {
    use crate::functions::compile_function::compile_function;

    // the example looks silly but we preserve it verbatim as it was found by fuzzer for a previous version of the compiler
    let actual = compile_function(
        "local function foo(a, b)\nend\nfoo(foo(foo,foo(foo,foo))[foo])",
        1,
        2,
        0,
    );
    let expected =
        "\nDUPCLOSURE R0 K0 ['foo']\nLOADNIL R3\nLOADNIL R2\nGETTABLE R1 R2 R0\nRETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // verify that invocations of the inlined function in any position for computing the arguments to itself compile
    let actual = compile_function(
        "local function foo(a, b)\n    return a + b\nend\n\nlocal x, y, z = ...\n\nreturn foo(foo(x, y), foo(z, 1))",
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 3\nADD R5 R1 R2\nADDK R6 R3 K1 [1]\nADD R4 R5 R6\nRETURN R4 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // verify that invocations of the inlined function in any position for computing the arguments to itself compile, including constants and locals
    // note that foo(k1, k2) doesn't get constant folded, so there's still actual math emitted for some of the calls below
    let actual = compile_function(
        "local function foo(a, b)\n    return a + b\nend\n\nlocal x, y, z = ...\n\nreturn\n    foo(foo(1, 2), 3),\n    foo(1, foo(2, 3)),\n    foo(x, foo(2, 3)),\n    foo(x, foo(y, 3)),\n    foo(x, foo(y, z)),\n    foo(x+0, foo(y, z)),\n    foo(x+0, foo(y+0, z)),\n    foo(x+0, foo(y, z+0)),\n    foo(1, foo(x, y))",
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 3\nLOADN R5 3\nADDK R4 R5 K1 [3]\nLOADN R6 5\nLOADN R7 1\nADD R5 R7 R6\nLOADN R7 5\nADD R6 R1 R7\nADDK R8 R2 K1 [3]\nADD R7 R1 R8\nADD R9 R2 R3\nADD R8 R1 R9\nADDK R10 R1 K2 [0]\nADD R11 R2 R3\nADD R9 R10 R11\nADDK R11 R1 K2 [0]\nADDK R13 R2 K2 [0]\nADD R12 R13 R3\nADD R10 R11 R12\nADDK R12 R1 K2 [0]\nADDK R14 R3 K2 [0]\nADD R13 R2 R14\nADD R11 R12 R13\nADD R13 R1 R2\nLOADN R14 1\nADD R12 R14 R13\nRETURN R4 9\n";
    assert_eq!(actual.trim(), expected.trim());
}
