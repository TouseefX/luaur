//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:90:unifier_2_string_x_y`
//! Source: `tests/Unifier2.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Unifier2.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Unifier2.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Unifier2.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item unifier_2_string_x_y

#[cfg(test)]
#[test]
fn unifier2_string_x_y() {
    use crate::records::unifier_2_fixture::Unifier2Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type_pack::follow_type_pack_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Unifier2Fixture::new();
    let string_to_unit = {
        let args = fixture
            .arena
            .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.stringType]);
        let rets = fixture.arena.add_type_pack_initializer_list_type_id(&[]);
        fixture
            .arena
            .add_type(FunctionType::function_type_new(args, rets, None, false))
    };

    let (x, x_free) = fixture.fresh_type();
    let y = fixture
        .arena
        .fresh_type_pack(&mut *fixture.scope, Polarity::Unknown);

    let x_to_y = {
        let args = fixture.arena.add_type_pack_initializer_list_type_id(&[x]);
        fixture
            .arena
            .add_type(FunctionType::function_type_new(args, y, None, false))
    };

    fixture.u2.unify(string_to_unit, x_to_y);

    assert_eq!(
        "string",
        fixture.to_string_type_id(unsafe { (*x_free).upper_bound })
    );

    let followed_y = unsafe { follow_type_pack_id(y) };
    let (head, tail) = flatten_type_pack_id(followed_y);

    assert_eq!(0, head.len());
    assert!(tail.is_none());
}
