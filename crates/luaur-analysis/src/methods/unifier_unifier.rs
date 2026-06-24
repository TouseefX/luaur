use crate::enums::variance::Variance;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::txn_log::TxnLog;
use crate::records::unifier::Unifier;
use luaur_ast::records::location::Location;

impl Unifier {
    pub fn unifier_unifier(
        _normalizer: *mut Normalizer,
        _scope: *mut Scope,
        _location: &Location,
        _variance: Variance,
        _parent_log: *mut TxnLog,
    ) {
        luaur_common::macros::luau_assert::LUAU_ASSERT!(false);
    }
}
