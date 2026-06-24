#[cfg(test)]
#[test]
fn parser_classes_can_interleave_methods_and_properties() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::ast_class_method::AstClassMethod;
    use luaur_ast::records::ast_class_property::AstClassProperty;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_class::AstStatClass;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = alloc::string::String::from(
        "\n        class Student\n            public name: string\n\n            function getname(self): string\n                return self.name:upper()\n            end\n\n            public year: number\n\n            function getyear(self): number\n                assert(self.year >= 1900 and self.year < 2100)\n                return self.year\n            end\n        end\n    ",
    );
    let res = fix.try_parse(&src, &ParseOptions::default());
    assert!(res.errors.is_empty());

    assert_eq!(unsafe { (*res.root).body.size }, 1);
    let s0 = unsafe { *(*res.root).body.data.add(0) };
    let cls = unsafe { luaur_ast::rtti::ast_node_as::<AstStatClass>(s0 as *mut AstNode) };
    assert!(!cls.is_null());
    let cls = unsafe { &*cls };
    assert!(unsafe { (*cls.name).name }.operator_eq_c_char(c"Student".as_ptr()));

    assert_eq!(cls.members.size, 4);

    let m1 = unsafe { &*cls.members.data.add(0) }.get_if::<AstClassProperty>();
    assert!(m1.is_some());
    assert!(m1.unwrap().name.operator_eq_c_char(c"name".as_ptr()));

    let m2 = unsafe { &*cls.members.data.add(1) }.get_if::<AstClassMethod>();
    assert!(m2.is_some());
    assert!(m2
        .unwrap()
        .function_name
        .operator_eq_c_char(c"getname".as_ptr()));

    let m3 = unsafe { &*cls.members.data.add(2) }.get_if::<AstClassProperty>();
    assert!(m3.is_some());
    assert!(m3.unwrap().name.operator_eq_c_char(c"year".as_ptr()));

    let m4 = unsafe { &*cls.members.data.add(3) }.get_if::<AstClassMethod>();
    assert!(m4.is_some());
    assert!(m4
        .unwrap()
        .function_name
        .operator_eq_c_char(c"getyear".as_ptr()));
}
