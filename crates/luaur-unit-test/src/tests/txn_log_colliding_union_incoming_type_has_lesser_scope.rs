//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TxnLog.test.cpp:36:txn_log_colliding_union_incoming_type_has_lesser_scope`
//! Source: `tests/TxnLog.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TxnLog.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TxnLog.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TxnLog.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - calls -> function log2 (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> method TxnLog::concatAsUnion (Analysis/src/TxnLog.cpp)
//!   - type_ref -> record PendingType (Analysis/include/Luau/TxnLog.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item txn_log_colliding_union_incoming_type_has_lesser_scope

#[cfg(test)]
#[test]
fn txn_log_colliding_union_incoming_type_has_lesser_scope() {
    use crate::records::txn_log_fixture::TxnLogFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::txn_log::TxnLog;
    use luaur_analysis::type_aliases::bound_type::BoundType;
    use luaur_analysis::type_aliases::type_id::TypeId;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = TxnLogFixture::new();

    fixture
        .log
        .replace_type_id_t(fixture.a, BoundType::bound_t(fixture.c));
    fixture
        .log2
        .replace_type_id_t(fixture.c, BoundType::bound_t(fixture.a));

    assert!(!fixture.log.pending_type_id(fixture.a).is_null());

    let log2 = core::mem::replace(&mut fixture.log2, TxnLog::new());
    fixture
        .log
        .concat_as_union(log2, &mut fixture.arena as *mut _);

    assert!(fixture.log.pending_type_id(fixture.a).is_null());

    let pending = fixture.log.pending_type_id(fixture.c);
    assert!(!pending.is_null());

    let bound = unsafe {
        fixture
            .log
            .txn_log_get::<BoundType, TypeId>(fixture.c)
            .as_ref()
    };
    assert_eq!(fixture.a, bound.unwrap().boundTo);

    fixture.log.commit();

    assert!(!unsafe { get_type_id::<FreeType>(fixture.a) }.is_null());

    let bound = unsafe { get_type_id::<BoundType>(fixture.c).as_ref() };
    assert_eq!(fixture.a, bound.unwrap().boundTo);
}
