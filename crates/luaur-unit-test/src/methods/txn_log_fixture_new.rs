use crate::records::txn_log_fixture::TxnLogFixture;
use alloc::string::String;
use alloc::sync::Arc;
use luaur_analysis::enums::polarity::Polarity;
use luaur_analysis::functions::fresh_type::fresh_type;
use luaur_analysis::records::builtin_types::BuiltinTypes;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::txn_log::TxnLog;
use luaur_analysis::records::type_arena::TypeArena;

impl TxnLogFixture {
    pub fn new() -> Self {
        let mut arena = TypeArena::default();
        let builtin_types = BuiltinTypes::new();
        let global_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.any_type_pack()));
        let child_scope = Arc::new(Scope::new(&global_scope, 0));

        let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
        let child_scope_ptr = Arc::as_ptr(&child_scope) as *mut Scope;

        let a = fresh_type(
            &mut arena,
            &builtin_types,
            global_scope_ptr,
            Polarity::Unknown,
        );
        let b = fresh_type(
            &mut arena,
            &builtin_types,
            global_scope_ptr,
            Polarity::Unknown,
        );
        let c = fresh_type(
            &mut arena,
            &builtin_types,
            child_scope_ptr,
            Polarity::Unknown,
        );

        let generic_name = String::from("G");
        let g = arena.add_type(GenericType::generic_type_name_polarity(
            &generic_name,
            Polarity::Mixed,
        ));

        Self {
            log: TxnLog::new(),
            log2: TxnLog::new(),
            arena,
            builtin_types,
            global_scope,
            child_scope,
            a,
            b,
            c,
            g,
        }
    }
}
