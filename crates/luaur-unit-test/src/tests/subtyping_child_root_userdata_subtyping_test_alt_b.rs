//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1010:subtyping_child_root_userdata`
//! Source: `tests/Subtyping.test.cpp`

use crate::records::subtype_fixture::SubtypeFixture;

#[cfg(test)]
#[test]
fn subtyping_child_root_userdata() {
    let mut fixture = SubtypeFixture::default();
    let hierarchy = fixture.class_hierarchy();
    let extern_ty = fixture.builtin_types.externType;
    let not_root = fixture.negate(hierarchy.root_class);
    let left = fixture.meet(hierarchy.child_class, not_root);

    assert!(fixture
        .is_subtype_type_id_type_id(left, extern_ty)
        .is_subtype());
}
