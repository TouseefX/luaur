#[cfg(test)]
#[test]
fn compiler_and_or() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("local a = 1 a = a and 2 return a");
    let expected = "\nLOADN R0 1\nANDK R0 R0 K0 [2]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 local b = ... a = a and b return a");
    let expected = "\nLOADN R0 1\nGETVARARGS R1 1\nAND R0 R0 R1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 b = 2 a = a and b return a");
    let expected = "\nLOADN R0 1\nLOADN R1 2\nSETGLOBAL R1 K0 ['b']\nMOVE R1 R0\nJUMPIFNOT R1 L0\nGETGLOBAL R1 K0 ['b']\nL0: MOVE R0 R1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 a = a or 2 return a");
    let expected = "\nLOADN R0 1\nORK R0 R0 K0 [2]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 local b = ... a = a or b return a");
    let expected = "\nLOADN R0 1\nGETVARARGS R1 1\nOR R0 R0 R1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 b = 2 a = a or b return a");
    let expected = "\nLOADN R0 1\nLOADN R1 2\nSETGLOBAL R1 K0 ['b']\nMOVE R1 R0\nJUMPIF R1 L0\nGETGLOBAL R1 K0 ['b']\nL0: MOVE R0 R1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 a = a b = 2 local c = a and b return c");
    let expected = "\nLOADN R0 1\nLOADN R1 2\nSETGLOBAL R1 K0 ['b']\nMOVE R1 R0\nJUMPIFNOT R1 L0\nGETGLOBAL R1 K0 ['b']\nL0: RETURN R1 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 a = a b = 2 local c = a or b return c");
    let expected = "\nLOADN R0 1\nLOADN R1 2\nSETGLOBAL R1 K0 ['b']\nMOVE R1 R0\nJUMPIF R1 L0\nGETGLOBAL R1 K0 ['b']\nL0: RETURN R1 1\n";
    assert_eq!(format!("\n{}", actual), expected);
}
