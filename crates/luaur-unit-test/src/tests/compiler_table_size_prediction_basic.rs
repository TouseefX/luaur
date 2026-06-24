#[cfg(test)]
#[test]
fn compiler_table_size_prediction_basic() {
    use crate::functions::compile_function_0::compile_function_0;

    let result1 = compile_function_0(
        "local t = {}\nt.a = 1\nt.b = 1\nt.c = 1\nt.d = 1\nt.e = 1\nt.f = 1\nt.g = 1\nt.h = 1\nt.i = 1",
    );
    let expected1 = "\nNEWTABLE R0 16 0\nLOADN R1 1\nSETTABLEKS R1 R0 K0 ['a']\nLOADN R1 1\nSETTABLEKS R1 R0 K1 ['b']\nLOADN R1 1\nSETTABLEKS R1 R0 K2 ['c']\nLOADN R1 1\nSETTABLEKS R1 R0 K3 ['d']\nLOADN R1 1\nSETTABLEKS R1 R0 K4 ['e']\nLOADN R1 1\nSETTABLEKS R1 R0 K5 ['f']\nLOADN R1 1\nSETTABLEKS R1 R0 K6 ['g']\nLOADN R1 1\nSETTABLEKS R1 R0 K7 ['h']\nLOADN R1 1\nSETTABLEKS R1 R0 K8 ['i']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function_0(
        "local t = {}\nt.x = 1\nt.x = 2\nt.x = 3\nt.x = 4\nt.x = 5\nt.x = 6\nt.x = 7\nt.x = 8\nt.x = 9",
    );
    let expected2 = "\nNEWTABLE R0 1 0\nLOADN R1 1\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 2\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 3\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 4\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 5\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 6\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 7\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 8\nSETTABLEKS R1 R0 K0 ['x']\nLOADN R1 9\nSETTABLEKS R1 R0 K0 ['x']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function_0(
        "local t = {}\nt[1] = 1\nt[2] = 1\nt[3] = 1\nt[4] = 1\nt[5] = 1\nt[6] = 1\nt[7] = 1\nt[8] = 1\nt[9] = 1\nt[10] = 1",
    );
    let expected3 = "\nNEWTABLE R0 0 10\nLOADN R1 1\nSETTABLEN R1 R0 1\nLOADN R1 1\nSETTABLEN R1 R0 2\nLOADN R1 1\nSETTABLEN R1 R0 3\nLOADN R1 1\nSETTABLEN R1 R0 4\nLOADN R1 1\nSETTABLEN R1 R0 5\nLOADN R1 1\nSETTABLEN R1 R0 6\nLOADN R1 1\nSETTABLEN R1 R0 7\nLOADN R1 1\nSETTABLEN R1 R0 8\nLOADN R1 1\nSETTABLEN R1 R0 9\nLOADN R1 1\nSETTABLEN R1 R0 10\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result3), expected3);
}
