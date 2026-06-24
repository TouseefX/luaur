#[cfg(test)]
#[test]
fn compiler_arithmetics() {
    use crate::functions::compile_function_0::compile_function_0;

    // basic arithmetics codegen with non-constants
    let actual =
        compile_function_0("local a, b = ...\nreturn a + b, a - b, a / b, a * b, a % b, a ^ b");
    let expected = "\nGETVARARGS R0 2\nADD R2 R0 R1\nSUB R3 R0 R1\nDIV R4 R0 R1\nMUL R5 R0 R1\nMOD R6 R0 R1\nPOW R7 R0 R1\nRETURN R2 6\n";
    assert_eq!(format!("\n{}", actual), expected);

    // basic arithmetics codegen with constants on the right sides
    // note that we don't simplify these expressions as we don't know the type of a
    let actual =
        compile_function_0("local a = ...\nreturn a + 1, a - 1, a / 1, a * 1, a % 1, a ^ 1");
    let expected = "\nGETVARARGS R0 1\nADDK R1 R0 K0 [1]\nSUBK R2 R0 K0 [1]\nDIVK R3 R0 K0 [1]\nMULK R4 R0 K0 [1]\nMODK R5 R0 K0 [1]\nPOWK R6 R0 K0 [1]\nRETURN R1 6\n";
    assert_eq!(format!("\n{}", actual), expected);
}
