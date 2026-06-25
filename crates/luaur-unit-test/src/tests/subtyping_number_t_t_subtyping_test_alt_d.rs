//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1565:subtyping_number_t_t`
//! Source: `tests/Subtyping.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Subtyping.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypePath.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/RegisterCallbacks.h
//! - incoming:
//!   - declares <- source_file tests/Subtyping.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item subtyping_number_t_t

use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;

#[cfg(test)]
#[test]
fn subtyping_number_t_t() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;
    let generic_t = fixture.generic("T");

    let number_to_nothing_ty =
        fixture.fn_item_initializer_list_type_id_initializer_list_type_id(vec![number_ty], vec![]);
    let generic_t_to_nothing_ty = fixture.generic_fn(vec![generic_t], vec![generic_t], vec![]);

    let f1 = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_to_nothing_ty],
        vec![],
    );
    let f2 = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![generic_t_to_nothing_ty],
        vec![],
    );

    assert!(fixture.is_subtype_type_id_type_id(f1, f2).is_subtype());
}
