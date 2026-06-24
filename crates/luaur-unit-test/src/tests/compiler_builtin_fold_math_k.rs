#[cfg(test)]
#[test]
fn compiler_builtin_fold_math_k() {
    use crate::functions::compile_function::compile_function;

    let test_cases = [
        ("pi", "6.2831853071795862"),
        ("e", "5.4365636569180902"),
        ("phi", "3.2360679774997898"),
        ("sqrt2", "2.8284271247461903"),
        ("tau", "12.566370614359172"),
    ];

    let replace_at_symbol_with_text =
        |source: &str, text: &str| -> String { source.replace('@', text) };

    for (constant, folded) in test_cases.iter() {
        // we can fold math constants at optimization level 2
        let source_code = replace_at_symbol_with_text(
            r#"
            function test()
                return @ * 2
            end
        "#,
            &format!("math.{}", constant),
        );
        let expected_bytecode_o2 =
            replace_at_symbol_with_text("LOADK R0 K0 [@]\nRETURN R0 1\n", folded);
        assert_eq!(
            compile_function(&source_code, 0, 2, 0),
            expected_bytecode_o2
        );

        // we don't do this at optimization level 1 because it may interfere with environment substitution
        let expected_bytecode_o1 = replace_at_symbol_with_text(
            "GETIMPORT R1 3 [math.@]\nMULK R0 R1 K0 [2]\nRETURN R0 1\n",
            constant,
        );
        assert_eq!(
            compile_function(&source_code, 0, 1, 0),
            expected_bytecode_o1
        );

        // we also don't do it if math global is assigned to
        let source_code_with_assignment = replace_at_symbol_with_text(
            r#"
            function test()
                return @ * 2
            end

            math = { pi = 4 }
        "#,
            &format!("math.{}", constant),
        );
        let expected_bytecode_with_assignment = replace_at_symbol_with_text(
            "GETGLOBAL R1 K1 ['math']\nGETTABLEKS R1 R1 K2 ['@']\nMULK R0 R1 K0 [2]\nRETURN R0 1\n",
            constant,
        );

        assert_eq!(
            compile_function(&source_code_with_assignment, 0, 2, 0),
            expected_bytecode_with_assignment
        );
    }
}
