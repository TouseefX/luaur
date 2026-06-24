//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:91:variant_non_pod`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item variant_non_pod

#[cfg(test)]
#[test]
fn variant_non_pod() {
    use alloc::string::String;
    use luaur_common::records::variant::Variant2;

    let s1 = String::from("hello");
    let v1: Variant2<String, i32> = Variant2::V0(s1.clone());

    assert_eq!(v1.get_if_0().unwrap(), "hello");

    let mut v2: Variant2<String, i32> = Variant2::V0(String::from("hello"));

    assert_eq!(v2.get_if_0().unwrap(), "hello");

    v2 = Variant2::V0(String::from(
        "this is a long string that doesn't fit into the small buffer",
    ));

    assert_eq!(
        v2.get_if_0().unwrap(),
        "this is a long string that doesn't fit into the small buffer"
    );

    let s2 = String::from("this is another long string, and this time we're copying it");
    v2 = Variant2::V0(s2.clone());

    assert_eq!(
        v2.get_if_0().unwrap(),
        "this is another long string, and this time we're copying it"
    );

    let mut v3 = v2.clone();

    assert_eq!(
        v2.get_if_0().unwrap(),
        "this is another long string, and this time we're copying it"
    );
    assert_eq!(
        v3.get_if_0().unwrap(),
        "this is another long string, and this time we're copying it"
    );

    let moved = core::mem::take(v3.get_if_0_mut().unwrap());
    let v4: Variant2<String, i32> = Variant2::V0(moved);

    assert_eq!(
        v2.get_if_0().unwrap(),
        "this is another long string, and this time we're copying it"
    );
    assert_eq!(v3.get_if_0().unwrap(), "");
    assert_eq!(
        v4.get_if_0().unwrap(),
        "this is another long string, and this time we're copying it"
    );
}
