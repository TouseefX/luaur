pub fn desugared_array_type_reference_is_empty<'a>(enabled: &'a str, disabled: &'a str) -> &'a str {
    if luaur_common::FFlag::DesugaredArrayTypeReferenceIsEmpty.get() {
        enabled
    } else {
        disabled
    }
}
