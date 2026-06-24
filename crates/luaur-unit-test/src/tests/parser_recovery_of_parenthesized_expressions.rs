#[cfg(test)]
#[test]
fn parser_recovery_of_parenthesized_expressions() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::visit::AstVisitable;

    impl luaur_ast::records::ast_visitor::AstVisitor for crate::records::count_ast_nodes::CountAstNodes {
        fn visit_node(&mut self, node: *mut core::ffi::c_void) -> bool {
            crate::records::count_ast_nodes::CountAstNodes::visit(
                self,
                node as *mut luaur_ast::records::ast_node::AstNode,
            )
        }
    }

    let mut fix = Fixture::default();
    let parse_options = ParseOptions::parse_options();

    let check_ast_equivalence = |fix: &mut Fixture, code_with_errors: &str, code: &str| {
        // Parse with errors, count AST nodes
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fix.parse(code_with_errors, &parse_options);
        }));

        let mut counter_with_errors = crate::records::count_ast_nodes::CountAstNodes::default();
        unsafe {
            let root = (*fix.source_module.as_ref().unwrap().as_ref()).root;
            (*root).visit(&mut counter_with_errors);
        }

        // Parse correct code, count AST nodes
        fix.parse(code, &parse_options);

        let mut counter = crate::records::count_ast_nodes::CountAstNodes::default();
        unsafe {
            let root = (*fix.source_module.as_ref().unwrap().as_ref()).root;
            (*root).visit(&mut counter);
        }

        assert_eq!(counter_with_errors.count, counter.count);
    };

    let mut check_recovery = |code_with_errors: &str, code: &str, expected_error_count: u32| {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fix.parse(code_with_errors, &parse_options);
        }));

        if result.is_ok() {
            panic!("Expected ParseErrors to be thrown");
        }

        let err = result.unwrap_err();
        let parse_errors = err.downcast_ref::<luaur_ast::records::parse_errors::ParseErrors>();
        if let Some(errors) = parse_errors {
            assert_eq!(errors.get_errors().len() as u32, expected_error_count);
        } else {
            panic!("Expected ParseErrors");
        }

        check_ast_equivalence(&mut fix, code_with_errors, code);
    };

    check_recovery(
        "function foo(a, b. c) return a + b end",
        "function foo(a, b) return a + b end",
        1,
    );

    check_recovery(
        "function foo(a, b: { a: number, b: number. c:number }) return a + b end",
        "function foo(a, b: { a: number, b: number }) return a + b end",
        1,
    );

    check_recovery(
        "function foo(a, b): (number -> number return a + b end",
        "function foo(a, b): (number) -> number return a + b end",
        1,
    );

    check_recovery(
        "function foo(a, b): (number, number -> number return a + b end",
        "function foo(a, b): (number) -> number return a + b end",
        1,
    );

    check_recovery(
        "function foo(a, b): (number; number) -> number return a + b end",
        "function foo(a, b): (number) -> number return a + b end",
        1,
    );

    check_recovery(
        "function foo(a, b): (number, number return a + b end",
        "function foo(a, b): (number, number) end",
        1,
    );

    check_recovery(
        "local function foo(a, b): (number, number return a + b end",
        "local function foo(a, b): (number, number) end",
        1,
    );

    check_recovery(
        "type F = (number, number -> number",
        "type F = (number, number) -> number",
        1,
    );

    check_recovery(
        "function foo(a, b: { a: number, b: number) return a + b end",
        "function foo(a, b: { a: number, b: number }) return a + b end",
        1,
    );

    check_recovery(
        "function foo(a, b: { [number: number}) return a + b end",
        "function foo(a, b: { [number]: number}) return a + b end",
        1,
    );

    check_recovery(
        "local n: (string | number = 2",
        "local n: (string | number) = 2",
        1,
    );

    check_recovery(
        "\nfunction foo(a, b\n    return a + b\nend\n",
        "function foo(a, b) return a + b end",
        1,
    );
}
