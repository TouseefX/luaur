#[cfg(test)]
#[test]
fn compiler_vector_arith_rev_k() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;

    // / has special optimized form for reverse constants; in absence of type information, we can't optimize other ops
    let actual = compile_function_0("local x: vector = ...\nreturn 2 * x, 2 / x, 2 // x");
    let expected = "\nGETVARARGS R0 1\nLOADN R2 2\nMUL R1 R2 R0\nDIVRK R2 K0 [2] R0\nLOADN R4 2\nIDIV R3 R4 R0\nRETURN R1 3\n";
    assert_eq!(format!("\n{}", actual), expected);

    // the same code with type information can optimize commutative operator * as well
    // other operators are not important enough to optimize reverse constant forms for
    let actual = compile_function(
        "local x: vector = ...\nreturn 2 * x, 2 / x, 2 // x",
        0,
        2,
        1,
    );
    let expected = "\nGETVARARGS R0 1\nMULK R1 R0 K0 [2]\nDIVRK R2 K0 [2] R0\nLOADN R4 2\nIDIV R3 R4 R0\nRETURN R1 3\n";
    assert_eq!(format!("\n{}", actual), expected);

    // vector components resolve to numbers which also allows reverse or transposed operations
    let actual = compile_function(
        "local x: vector = ...\nreturn 2 + x.x, 2 - x.x, 2 * x.x, 2 / x.x, 2 + x.Y, 2 - x.Y, 2 * x.Y, 2 / x.Y",
        0,
        2,
        1,
    );
    let expected = "\nGETVARARGS R0 1\nGETTABLEKS R2 R0 K1 ['x']\nADDK R1 R2 K0 [2]\nGETTABLEKS R3 R0 K1 ['x']\nSUBRK R2 K0 [2] R3\nGETTABLEKS R4 R0 K1 ['x']\nMULK R3 R4 K0 [2]\nGETTABLEKS R5 R0 K1 ['x']\nDIVRK R4 K0 [2] R5\nGETTABLEKS R6 R0 K2 ['Y']\nADDK R5 R6 K0 [2]\nGETTABLEKS R7 R0 K2 ['Y']\nSUBRK R6 K0 [2] R7\nGETTABLEKS R8 R0 K2 ['Y']\nMULK R7 R8 K0 [2]\nGETTABLEKS R9 R0 K2 ['Y']\nDIVRK R8 K0 [2] R9\nRETURN R1 8\n";
    assert_eq!(format!("\n{}", actual), expected);
}
