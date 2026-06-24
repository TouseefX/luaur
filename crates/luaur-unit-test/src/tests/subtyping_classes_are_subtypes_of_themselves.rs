use crate::records::subtype_fixture::SubtypeFixture;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use luaur_common::FFlag;

#[cfg(test)]
#[test]
fn subtyping_classes_are_subtypes_of_themselves() {
    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let mut fixture = SubtypeFixture::default();
    let a = fixture.user_defined_cls("A", None);
    let b = fixture.user_defined_cls("B", None);

    assert!(fixture.is_subtype_type_id_type_id(a, a).is_subtype());
    assert!(fixture.is_subtype_type_id_type_id(b, b).is_subtype());
}
