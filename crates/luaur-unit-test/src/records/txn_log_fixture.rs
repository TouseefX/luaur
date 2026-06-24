use luaur_analysis::records::builtin_types::BuiltinTypes;
use luaur_analysis::records::txn_log::TxnLog;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::type_aliases::scope_ptr_type::ScopePtr;
use luaur_analysis::type_aliases::type_id::TypeId;

#[derive(Debug)]
pub struct TxnLogFixture {
    pub log: TxnLog,
    pub log2: TxnLog,
    pub arena: TypeArena,
    pub builtin_types: BuiltinTypes,
    pub global_scope: ScopePtr,
    pub child_scope: ScopePtr,
    pub a: TypeId,
    pub b: TypeId,
    pub c: TypeId,
    pub g: TypeId,
}
