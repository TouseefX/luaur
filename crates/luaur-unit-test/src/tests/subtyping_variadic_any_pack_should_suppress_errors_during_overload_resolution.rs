//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1873:subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution`
//! Source: `tests/Subtyping.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Subtyping.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypePath.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/RegisterCallbacks.h
//! - incoming:
//!   - declares <- source_file tests/Subtyping.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution

#[cfg(test)]
#[test]
fn subtyping_variadic_any_pack_should_suppress_errors_during_overload_resolution() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let res = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type ActionCallback = (string) -> ...any

function bindAction(callback: ActionCallback)
  local _ = function(...)
    callback(...)
  end
end
"#,
        ),
        None,
    );

    fixture.validate_errors(&res.errors);
    assert!(res.errors.is_empty(), "{}", fixture.get_errors(&res));
}
