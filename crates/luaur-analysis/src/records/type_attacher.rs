use crate::records::module::Module;
use crate::type_aliases::synthetic_names::SyntheticNames;
use luaur_ast::records::allocator::Allocator;

#[derive(Debug, Clone)]
pub struct TypeAttacher {
    pub(crate) module: *mut Module,
    pub(crate) allocator: *mut Allocator,
    pub(crate) synthetic_names: SyntheticNames,
}
