use crate::records::def::Def;
use crate::records::symbol::Symbol;
use crate::records::typed_allocator::TypedAllocator;
use crate::type_aliases::def_id_refinement::DefId;
use luaur_ast::records::location::Location;

#[derive(Debug)]
pub struct DefArena {
    pub allocator: TypedAllocator<Def>,
}

impl Default for DefArena {
    fn default() -> Self {
        Self {
            allocator: TypedAllocator::typed_allocator(),
        }
    }
}
