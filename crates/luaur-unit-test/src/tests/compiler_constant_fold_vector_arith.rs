#[cfg(test)]
#[test]
fn compiler_constant_fold_vector_arith() {
    use crate::functions::compile_function::compile_function;

    let actual1 = compile_function(
        "local n = 2; local a, b = vector.create(1, 2, 3), vector.create(2, 4, 8); return a + b",
        0,
        2,
        0,
    );
    let expected1 = "\nLOADK R0 K0 [3, 6, 11]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function(
        "local n = 2; local a, b = vector.create(1, 2, 3), vector.create(2, 4, 8); return a - b",
        0,
        2,
        0,
    );
    let expected2 = "\nLOADK R0 K0 [-1, -2, -5]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function(
        "local n = 2; local a, b = vector.create(1, 2, 3), vector.create(2, 4, 8); return a * n, a * b, n * b, a * math.huge",
        0, 2, 0,
    );
    let expected3 = "\nLOADK R0 K0 [2, 4, 6]\nLOADK R1 K1 [2, 8, 24]\nLOADK R2 K2 [4, 8, 16]\nLOADK R4 K4 [1, 2, 3]\nMULK R3 R4 K3 [inf]\nRETURN R0 4\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    let actual4 = compile_function(
        "local n = 2; local a, b = vector.create(1, 2, 3), vector.create(2, 4, 8); return a / n, a / b, n / b, a / math.huge",
        0, 2, 0,
    );
    let expected4 = "\nLOADK R0 K0 [0.5, 1, 1.5]\nLOADK R2 K1 [1, 2, 3]\nLOADK R3 K2 [2, 4, 8]\nDIV R1 R2 R3\nLOADK R3 K2 [2, 4, 8]\nDIVRK R2 K3 [2] R3\nLOADK R3 K4 [0, 0, 0]\nRETURN R0 4\n";
    assert_eq!(format!("\n{}", actual4), expected4);

    let actual5 = compile_function(
        "local n = 2; local a, b = vector.create(1, 2, 3), vector.create(2, 4, 8); return a // n, a // b, n // b",
        0, 2, 0,
    );
    let expected5 = "\nLOADK R0 K0 [0, 1, 1]\nLOADK R2 K1 [1, 2, 3]\nLOADK R3 K2 [2, 4, 8]\nIDIV R1 R2 R3\nLOADN R3 2\nLOADK R4 K2 [2, 4, 8]\nIDIV R2 R3 R4\nRETURN R0 3\n";
    assert_eq!(format!("\n{}", actual5), expected5);

    let actual6 = compile_function("local a = vector.create(1, 2, 3); return -a", 0, 2, 0);
    let expected6 = "\nLOADK R0 K0 [-1, -2, -3]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual6), expected6);
}
