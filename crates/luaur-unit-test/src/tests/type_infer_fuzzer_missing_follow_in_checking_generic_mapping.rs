//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2893:type_infer_fuzzer_missing_follow_in_checking_generic_mapping`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_fuzzer_missing_follow_in_checking_generic_mapping

#[cfg(test)]
#[test]
fn type_infer_fuzzer_missing_follow_in_checking_generic_mapping() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function _<U...,M...>(l0,l0,l0,l0,)
            l0(_(rshift),_()(_(if _ then _),))
            _()(_(_(_)))
        end
        _()(_()(_(true,_)),)
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function _<Y...,U...,M...>(l0:any,l0,l0,...)
            _()(_,_()(_(_()),_))
            do end
        end
        do end
        _()(_(""),{})
        do end
        for _ in ... do
        end
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
