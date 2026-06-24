//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_stringifying_cyclic_intersection_type_bails_early() {
    use alloc::vec;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut arena = TypeArena::default();
    let tv = arena.add_type(IntersectionType { parts: vec![] });
    let itv = unsafe { get_mutable_type_id::<IntersectionType>(tv).as_mut() }
        .expect("expected intersection type");
    itv.parts.push(tv);
    itv.parts.push(tv);

    assert_eq!("t1 where t1 = t1 & t1", to_string_type_id(tv));
}
