#[cfg(test)]
#[test]
fn parser_class_declaration() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_class_property::AstClassProperty;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_local::AstExprLocal;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_class::AstStatClass;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = alloc::string::String::from(
        "\n        class Point2\n            public x: number\n            public y: number\n        end\n        print(Point2)\n    ",
    );
    let res = fix.try_parse(&src, &ParseOptions::default());
    assert!(res.errors.is_empty());

    assert_eq!(unsafe { (*res.root).body.size }, 2);

    let s0 = unsafe { *(*res.root).body.data.add(0) };
    let first = unsafe { luaur_ast::rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!first.is_null());
    let first = unsafe { &*first };
    assert!(unsafe { (*first.name).name }.operator_eq_c_char(c"Point2".as_ptr()));

    assert_eq!(first.members.size, 2);

    let m1 = unsafe { &*first.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"x".as_ptr()));

    let m2 = unsafe { &*first.members.data.add(1) }.get_if::<AstClassProperty>();
    assert!(m2.is_some());
    assert!(m2.unwrap().name.operator_eq_c_char(c"y".as_ptr()));

    let s1 = unsafe { *(*res.root).body.data.add(1) };
    let second = unsafe { luaur_ast::rtti::ast_node_as::<AstStatExpr>(s1 as *mut AstNode) };
    assert!(!second.is_null());
    let second = unsafe { &*second };

    let call = unsafe { luaur_ast::rtti::ast_node_as::<AstExprCall>(second.expr as *mut AstNode) };
    assert!(!call.is_null());
    let call = unsafe { &*call };

    assert_eq!(call.args.size, 1);
    let a0 = unsafe { *call.args.data.add(0) };
    let local = unsafe { luaur_ast::rtti::ast_node_as::<AstExprLocal>(a0 as *mut AstNode) };
    assert!(!local.is_null());
    let local = unsafe { &*local };
    assert!(local.local == first.name);
}
