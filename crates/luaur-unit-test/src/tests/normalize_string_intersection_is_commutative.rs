//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:517:normalize_string_intersection_is_commutative`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item normalize_string_intersection_is_commutative

#[cfg(test)]
#[test]
fn normalize_string_intersection_is_commutative() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = NormalizeFixture::default();

    let c4 = to_string_type_id(fixture.normal(
        r#"
        string & (string & Not<"a"> & Not<"b">)
"#,
    ));
    let c4_reverse = to_string_type_id(fixture.normal(
        r#"
        (string & Not<"a"> & Not<"b">) & string
"#,
    ));
    assert_eq!(c4, c4_reverse);
    assert_eq!(r#"string & ~"a" & ~"b""#, c4);

    let c5 = to_string_type_id(fixture.normal(
        r#"
        (string & Not<"a"> & Not<"b">) & (string & Not<"b"> & Not<"c">)
"#,
    ));
    let c5_reverse = to_string_type_id(fixture.normal(
        r#"
        (string & Not<"b"> & Not<"c">) & (string & Not<"a"> & Not<"c">)
"#,
    ));
    assert_eq!(c5, c5_reverse);
    assert_eq!(r#"string & ~"a" & ~"b" & ~"c""#, c5);

    let c6 = to_string_type_id(fixture.normal(
        r#"
        ("a" | "b") & (string & Not<"b"> & Not<"c">)
"#,
    ));
    let c6_reverse = to_string_type_id(fixture.normal(
        r#"
        (string & Not<"b"> & Not<"c">) & ("a" | "b")
"#,
    ));
    assert_eq!(c6, c6_reverse);
    assert_eq!(r#""a""#, c6);

    let c7 = to_string_type_id(fixture.normal(
        r#"
        string & ("b" | "c")
"#,
    ));
    let c7_reverse = to_string_type_id(fixture.normal(
        r#"
        ("b" | "c") & string
"#,
    ));
    assert_eq!(c7, c7_reverse);
    assert_eq!(r#""b" | "c""#, c7);

    let c8 = to_string_type_id(fixture.normal(
        r#"
(string & Not<"a"> & Not<"b">) & ("b" | "c")
"#,
    ));
    let c8_reverse = to_string_type_id(fixture.normal(
        r#"
        ("b" | "c") & (string & Not<"a"> & Not<"b">)
"#,
    ));
    assert_eq!(c8, c8_reverse);
    assert_eq!(r#""c""#, c8);

    let c9 = to_string_type_id(fixture.normal(
        r#"
            ("a" | "b") & ("b" | "c")
    "#,
    ));
    let c9_reverse = to_string_type_id(fixture.normal(
        r#"
            ("b" | "c") & ("a" | "b")
    "#,
    ));
    assert_eq!(c9, c9_reverse);
    assert_eq!(r#""b""#, c9);

    let l = to_string_type_id(fixture.normal(
        r#"
         (string | number) & ("a" | true)
    "#,
    ));
    let r = to_string_type_id(fixture.normal(
        r#"
         ("a" | true) & (string | number)
    "#,
    ));
    assert_eq!(l, r);
    assert_eq!(r#""a""#, l);
}
