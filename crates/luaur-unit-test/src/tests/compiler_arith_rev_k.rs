#[cfg(test)]
#[test]
fn compiler_arith_rev_k() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;

    // - and / have special optimized form for reverse constants; in absence of type information, we can't optimize other ops
    let actual = compile_function_0(
        "local x: number = unknown\nreturn 2 + x, 2 - x, 2 * x, 2 / x, 2 % x, 2 // x, 2 ^ x",
    );
    let expected = "\nGETIMPORT R0 1 [unknown]\nLOADN R2 2\nADD R1 R2 R0\nSUBRK R2 K2 [2] R0\nLOADN R4 2\nMUL R3 R4 R0\nDIVRK R4 K2 [2] R0\nLOADN R6 2\nMOD R5 R6 R0\nLOADN R7 2\nIDIV R6 R7 R0\nLOADN R8 2\nPOW R7 R8 R0\nRETURN R1 7\n";
    assert_eq!(format!("\n{}", actual), expected);

    // the same code with type information can optimize commutative operators (+ and *) as well
    // other operators are not important enough to optimize reverse constant forms for
    let actual = compile_function(
        "local x: number = unknown\nreturn 2 + x, 2 - x, 2 * x, 2 / x, 2 % x, 2 // x, 2 ^ x",
        0,
        2,
        1,
    );
    let expected = "\nGETIMPORT R0 1 [unknown]\nADDK R1 R0 K2 [2]\nSUBRK R2 K2 [2] R0\nMULK R3 R0 K2 [2]\nDIVRK R4 K2 [2] R0\nLOADN R6 2\nMOD R5 R6 R0\nLOADN R7 2\nIDIV R6 R7 R0\nLOADN R8 2\nPOW R7 R8 R0\nRETURN R1 7\n";
    assert_eq!(format!("\n{}", actual), expected);
}
