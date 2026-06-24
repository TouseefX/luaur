use crate::records::pending_expansion_type::PendingExpansionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_name::AstName;

impl PendingExpansionType {
    pub fn pending_expansion_type_pending_expansion_type(
        prefix: Option<AstName>,
        name: AstName,
        type_arguments: alloc::vec::Vec<TypeId>,
        pack_arguments: alloc::vec::Vec<TypePackId>,
    ) -> Self {
        Self {
            prefix,
            name,
            type_arguments,
            pack_arguments,
            index: Self::fresh_index(),
        }
    }
}
