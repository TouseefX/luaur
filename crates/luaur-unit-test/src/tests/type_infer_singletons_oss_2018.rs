//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:800:type_infer_singletons_oss_2018`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_singletons_oss_2018

#[cfg(test)]
#[test]
fn type_infer_singletons_oss_2018() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(r#"
        local rule: { rule: "AppendTextComment" } | { rule: "Other" } = { rule = "AppendTextComment" }
    "#),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
