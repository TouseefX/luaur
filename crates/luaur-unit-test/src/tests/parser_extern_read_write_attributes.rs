#[cfg(test)]
#[test]
fn parser_extern_read_write_attributes() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::enums::ast_table_access::AstTableAccess;
    use luaur_ast::records::ast_array::AstArray;
    use luaur_ast::records::ast_declared_extern_type_property::AstDeclaredExternTypeProperty;
    use luaur_ast::records::ast_name::AstName;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
    use luaur_ast::records::ast_table_indexer::AstTableIndexer;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _ = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\n        declare extern type Foo with\n            read ReadOnlyMember: string\n            write WriteOnlyMember: number\n            ReadWriteMember: vector\n            wRITE BadAttributeMember: buffer\n        end\n    ",
    );

    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].get_location().begin.line, 5);
    assert_eq!(
        *result.errors[0].get_message(),
        "Expected blank or 'read' or 'write' attribute, got 'wRITE'"
    );

    let stat: *mut AstStatBlock = result.root;
    assert_eq!(unsafe { (*stat).body.size }, 1);

    let declared_extern_type: *mut AstStatDeclareExternType = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatDeclareExternType>(
            *(*stat).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!declared_extern_type.is_null());
    assert_eq!(unsafe { (*declared_extern_type).props.size }, 4);

    let props_ptr = unsafe { (*declared_extern_type).props.data };
    assert_eq!(unsafe { (*props_ptr.add(0)).access }, AstTableAccess::Read);
    assert_eq!(unsafe { (*props_ptr.add(1)).access }, AstTableAccess::Write);
    assert_eq!(
        unsafe { (*props_ptr.add(2)).access },
        AstTableAccess::ReadWrite
    );
    assert_eq!(
        unsafe { (*props_ptr.add(3)).access },
        AstTableAccess::ReadWrite
    );
}
