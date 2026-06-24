#[cfg(test)]
#[test]
fn compiler_multiple_assignments() {
    use crate::functions::compile_function_0::compile_function_0;

    // order of assignments is left to right
    let result = compile_function_0("local a, b\na, b = f(1), f(2)");
    let expected = "\nLOADNIL R0\nLOADNIL R1\nGETIMPORT R2 1 [f]\nLOADN R3 1\nCALL R2 1 1\nMOVE R0 R2\nGETIMPORT R2 1 [f]\nLOADN R3 2\nCALL R2 1 1\nMOVE R1 R2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // this includes table assignments
    let result = compile_function_0("local t\nt[1], t[2] = 3, 4");
    let expected = "\nLOADNIL R0\nLOADNIL R1\nLOADN R2 3\nLOADN R3 4\nSETTABLEN R2 R0 1\nSETTABLEN R3 R1 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // semantically, we evaluate the right hand side first; this allows us to e.g swap elements in a table easily
    let result = compile_function_0("local t = ...\nt[1], t[2] = t[2], t[1]");
    let expected = "\nGETVARARGS R0 1\nGETTABLEN R1 R0 2\nGETTABLEN R2 R0 1\nSETTABLEN R1 R0 1\nSETTABLEN R2 R0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // however, we need to optimize local assignments; to do this well, we need to handle assignment conflicts
    // let's first go through a few cases where there are no conflicts:

    // when multiple assignments have no conflicts (all local vars are read after being assigned), codegen is the same as a series of single
    // assignments
    let result =
        compile_function_0("local xm1, x, xp1, xi = ...\n\nxm1,x,xp1,xi = x,xp1,xp1+1,xi-1");
    let expected = "\nGETVARARGS R0 4\nMOVE R0 R1\nMOVE R1 R2\nADDK R2 R2 K0 [1]\nSUBK R3 R3 K0 [1]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // similar example to above from a more complex case
    let result = compile_function_0(
        "local a, b, c, d, e, f, g, h, t1, t2 = ...\n\nh, g, f, e, d, c, b, a = g, f, e, d + t1, c, b, a, t1 + t2",
    );
    let expected = "\nGETVARARGS R0 10\nMOVE R7 R6\nMOVE R6 R5\nMOVE R5 R4\nADD R4 R3 R8\nMOVE R3 R2\nMOVE R2 R1\nMOVE R1 R0\nADD R0 R8 R9\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // when locals have a conflict, we assign temporaries instead of locals, and at the end copy the values back
    // the basic example of this is a swap/rotate
    let result = compile_function_0("local a, b = ...\na, b = b, a");
    let expected = "\nGETVARARGS R0 2\nMOVE R2 R1\nMOVE R1 R0\nMOVE R0 R2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("local a, b, c = ...\na, b, c = c, a, b");
    let expected = "\nGETVARARGS R0 3\nMOVE R3 R2\nMOVE R4 R0\nMOVE R2 R1\nMOVE R0 R3\nMOVE R1 R4\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("local a, b, c = ...\na, b, c = b, c, a");
    let expected =
        "\nGETVARARGS R0 3\nMOVE R3 R1\nMOVE R1 R2\nMOVE R2 R0\nMOVE R0 R3\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // multiple assignments with multcall handling - foo() evalutes to temporary registers and they are copied out to target
    let result = compile_function_0("local a, b, c, d = ...\na, b, c, d = 1, foo()");
    let expected = "\nGETVARARGS R0 4\nLOADN R0 1\nGETIMPORT R4 1 [foo]\nCALL R4 0 3\nMOVE R1 R4\nMOVE R2 R5\nMOVE R3 R6\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // note that during this we still need to handle local reassignment, eg when table assignments are performed
    let result = compile_function_0("local a, b, c, d = ...\na, b[a], c[d], d = 1, foo()");
    let expected = "\nGETVARARGS R0 4\nLOADN R4 1\nGETIMPORT R6 1 [foo]\nCALL R6 0 3\nSETTABLE R6 R1 R0\nSETTABLE R7 R2 R3\nMOVE R0 R4\nMOVE R3 R8\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // multiple assignments with multcall handling - foo evaluates to a single argument so all remaining locals are assigned to nil
    // note that here we don't assign the locals directly, as this case is very rare so we use the similar code path as above
    let result = compile_function_0("local a, b, c, d = ...\na, b, c, d = 1, foo");
    let expected = "\nGETVARARGS R0 4\nLOADN R0 1\nGETIMPORT R4 1 [foo]\nLOADNIL R5\nLOADNIL R6\nMOVE R1 R4\nMOVE R2 R5\nMOVE R3 R6\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // note that we also try to use locals as a source of assignment directly when assigning fields; this works using old local value when possible
    let result = compile_function_0("local a, b = ...\na[1], a[2] = b, b + 1");
    let expected =
        "\nGETVARARGS R0 2\nADDK R2 R1 K0 [1]\nSETTABLEN R1 R0 1\nSETTABLEN R2 R0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // ... of course if the local is reassigned, we defer the assignment until later
    let result = compile_function_0("local a, b = ...\nb, a[1] = 42, b");
    let expected = "\nGETVARARGS R0 2\nLOADN R2 42\nSETTABLEN R1 R0 1\nMOVE R1 R2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // when there are more expressions when values, we evalute them for side effects, but they also participate in conflict handling
    let result = compile_function_0("local a, b = ...\na, b = 1, 2, a + b");
    let expected = "\nGETVARARGS R0 2\nLOADN R2 1\nLOADN R3 2\nADD R4 R0 R1\nMOVE R0 R2\nMOVE R1 R3\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // because we perform assignments to complex l-values after assignments to locals, we make sure register conflicts are tracked accordingly
    let result = compile_function_0("local a, b = ...\na[1], b = b, b + 1");
    let expected =
        "\nGETVARARGS R0 2\nADDK R2 R1 K0 [1]\nSETTABLEN R1 R0 1\nMOVE R1 R2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);
}
