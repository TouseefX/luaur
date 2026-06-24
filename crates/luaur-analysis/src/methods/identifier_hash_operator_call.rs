use crate::records::identifier::Identifier;
use crate::records::identifier_hash::IdentifierHash;
use core::hash::{Hash, Hasher};

impl IdentifierHash {
    pub fn identifier_hash_operator_call(ident: &Identifier) -> usize {
        let name = ident.name();
        let ctx = ident.ctx() as *const ();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        name.hash(&mut hasher);
        let hash_name = hasher.finish() as usize;

        hasher = std::collections::hash_map::DefaultHasher::new();
        (ctx as usize).hash(&mut hasher);
        let hash_ctx = hasher.finish() as usize;

        hash_name ^ hash_ctx
    }
}
