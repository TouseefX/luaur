//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1257:frontend_module_scope_check`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item frontend_module_scope_check

#[cfg(test)]
#[test]
fn frontend_module_scope_check() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::ffi::CString;
    use alloc::rc::Rc;
    use alloc::string::String;
    use alloc::sync::Arc;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::binding::Binding;
    use luaur_analysis::records::frontend::Frontend;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::symbol::Symbol;
    use luaur_ast::records::ast_name::AstName;
    use luaur_ast::records::location::Location;

    let mut fixture = BuiltinsFixture::default();

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    unsafe {
        let number_type = (*(*frontend_ptr).builtin_types).numberType;
        let x = CString::new("x").unwrap();

        (*frontend_ptr).prepare_module_scope = Some(Rc::new(move |_, scope, _| {
            let name = AstName { value: x.as_ptr() };
            let scope_ptr = Arc::as_ptr(scope) as *mut Scope;
            unsafe {
                (*scope_ptr).bindings.insert(
                    Symbol::from_global(name),
                    Binding {
                        type_id: number_type,
                        location: Location::default(),
                        deprecated: false,
                        deprecated_suggestion: String::new(),
                        documentation_symbol: None,
                    },
                );
            }
        }));
    }

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        local a = x
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture
        .base
        .require_type_module_name_string("game/A", &String::from("a"));
    assert_eq!("number", to_string_type_id(ty));
}
