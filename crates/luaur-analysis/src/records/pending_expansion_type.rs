use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_name::AstName;

/// Global counter for assigning unique indices to `PendingExpansionType` instances.
/// Mirrors `int PendingExpansionType::nextIndex` in `Analysis/include/Luau/Type.h`.
static NEXT_INDEX: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PendingExpansionType {
    pub prefix: Option<AstName>,
    pub name: AstName,
    pub type_arguments: alloc::vec::Vec<TypeId>,
    pub pack_arguments: alloc::vec::Vec<TypePackId>,
    pub index: usize,
}

impl PendingExpansionType {
    pub fn fresh_index() -> usize {
        NEXT_INDEX.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
    }
}
