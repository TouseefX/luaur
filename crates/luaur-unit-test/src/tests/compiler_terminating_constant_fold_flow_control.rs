#[cfg(test)]
#[test]
fn compiler_terminating_constant_fold_flow_control() {
    use crate::functions::compile_function_0::compile_function_0;

    // if true then return
    let actual1 =
        compile_function_0("if true then\n    return 42\nend\n\nprint(\"not reachable\")");
    let expected1 = "\nLOADN R0 42\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // if false then else return
    let actual2 = compile_function_0("if false then\n    print(\"not seen\")\nelse\n    return 42\nend\n\nprint(\"not reachable\")");
    let expected2 = "\nLOADN R0 42\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    // nested do block with if true return
    let actual3 = compile_function_0(
        "do\n    if true then\n        return 42\n    end\nend\n\nprint(\"not reachable\")",
    );
    let expected3 = "\nLOADN R0 42\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    // nested do block with if true return and if false
    let actual4 = compile_function_0("do\n    if true then\n        return 42\n    end\n\n    if false then\n        print(\"not seen\")\n    end\nend\n\nprint(\"not reachable\")");
    let expected4 = "\nLOADN R0 42\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual4), expected4);

    // while true with break
    let actual5 = compile_function_0("while true do\n    if true then\n        break\n    end\n\n    print(\"unreachable\")\nend\n\nreturn 42");
    let expected5 = "\nJUMP L0\nJUMPBACK L0\nL0: LOADN R0 42\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual5), expected5);

    // while true with return
    let actual6 = compile_function_0(
        "while true do\n    if true then\n        return 42\n    end\n\n    print(\"unseen\")\nend",
    );
    let expected6 = "\nL0: LOADN R0 42\nRETURN R0 1\nJUMPBACK L0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual6), expected6);
}
