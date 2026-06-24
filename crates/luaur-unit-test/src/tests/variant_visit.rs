//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:170:variant_visit`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ToStringVisitor (tests/Variant.test.cpp)
//!   - type_ref -> record IncrementVisitor (tests/Variant.test.cpp)
//!   - translates_to -> rust_item variant_visit

#[cfg(test)]
#[test]
fn variant_visit() {
    use crate::records::increment_visitor::IncrementVisitor;
    use crate::records::to_string_visitor::ToStringVisitor;
    use alloc::string::String;
    use luaur_common::records::variant::Variant2;

    let mut v1: Variant2<String, i32> = Variant2::V0(String::from("123"));
    let mut v2: Variant2<String, i32> = Variant2::V1(45);
    let v1c = &v1;
    let v2c = &v2;

    let to_string_visitor = ToStringVisitor::default();
    let increment_visitor = IncrementVisitor::default();

    let mut r1 = String::new();
    match v1c {
        Variant2::V0(v) => r1.push_str(&to_string_visitor.operator_call(v)),
        Variant2::V1(v) => r1.push_str(&to_string_visitor.operator_call_mut(*v)),
    }
    match v2c {
        Variant2::V0(v) => r1.push_str(&to_string_visitor.operator_call(v)),
        Variant2::V1(v) => r1.push_str(&to_string_visitor.operator_call_mut(*v)),
    }
    assert_eq!(r1, "12345");

    let mut r2 = String::new();
    r2.push_str(&match v1c {
        Variant2::V0(v) => to_string_visitor.operator_call(v),
        Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
    });
    r2.push_str(&match v2c {
        Variant2::V0(v) => to_string_visitor.operator_call(v),
        Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
    });
    assert_eq!(r2, "12345");

    match &mut v1 {
        Variant2::V0(v) => increment_visitor.operator_call(v),
        Variant2::V1(v) => increment_visitor.operator_call_mut(v),
    }
    match &mut v2 {
        Variant2::V0(v) => increment_visitor.operator_call(v),
        Variant2::V1(v) => increment_visitor.operator_call_mut(v),
    }
    assert_eq!(
        match &v1 {
            Variant2::V0(v) => to_string_visitor.operator_call(v),
            Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
        },
        "1231"
    );
    assert_eq!(
        match &v2 {
            Variant2::V0(v) => to_string_visitor.operator_call(v),
            Variant2::V1(v) => to_string_visitor.operator_call_mut(*v),
        },
        "46"
    );

    let mut r3 = String::new();
    r3.push_str(&match &mut v1 {
        Variant2::V0(v) => {
            increment_visitor.operator_call(v);
            to_string_visitor.operator_call(v)
        }
        Variant2::V1(v) => {
            increment_visitor.operator_call_mut(v);
            to_string_visitor.operator_call_mut(*v)
        }
    });
    r3.push_str(&match &mut v2 {
        Variant2::V0(v) => {
            increment_visitor.operator_call(v);
            to_string_visitor.operator_call(v)
        }
        Variant2::V1(v) => {
            increment_visitor.operator_call_mut(v);
            to_string_visitor.operator_call_mut(*v)
        }
    });
    assert_eq!(r3, "1231147");
}
