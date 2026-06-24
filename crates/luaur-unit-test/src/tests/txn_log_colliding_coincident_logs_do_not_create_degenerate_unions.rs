//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TxnLog.test.cpp:69:txn_log_colliding_coincident_logs_do_not_create_degenerate_unions`
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
//!   - translates_to -> rust_item txn_log_colliding_coincident_logs_do_not_create_degenerate_unions

#[cfg(test)]
#[test]
fn txn_log_colliding_coincident_logs_do_not_create_degenerate_unions() {
    use crate::records::txn_log_fixture::TxnLogFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::txn_log::TxnLog;
    use luaur_analysis::type_aliases::bound_type::BoundType;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = TxnLogFixture::new();

    fixture
        .log
        .replace_type_id_t(fixture.a, BoundType::bound_t(fixture.b));
    fixture
        .log2
        .replace_type_id_t(fixture.a, BoundType::bound_t(fixture.b));

    let log2 = core::mem::replace(&mut fixture.log2, TxnLog::new());
    fixture
        .log
        .concat_as_union(log2, &mut fixture.arena as *mut _);

    fixture.log.commit();

    assert_eq!("'a", to_string_type_id(fixture.a));
    assert_eq!("'a", to_string_type_id(fixture.b));
}
