//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TxnLog.test.cpp:84:txn_log_replacing_persistent_types_is_allowed_but_makes_the_log_radioactive`
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
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item txn_log_replacing_persistent_types_is_allowed_but_makes_the_log_radioactive

#[cfg(test)]
#[test]
fn txn_log_replacing_persistent_types_is_allowed_but_makes_the_log_radioactive() {
    use crate::records::txn_log_fixture::TxnLogFixture;
    use luaur_analysis::functions::persist_type::persist;
    use luaur_analysis::type_aliases::bound_type::BoundType;

    let mut fixture = TxnLogFixture::new();

    persist(fixture.g);

    fixture
        .log
        .replace_type_id_t(fixture.g, BoundType::bound_t(fixture.a));

    assert!(fixture.log.is_radioactive());
}
