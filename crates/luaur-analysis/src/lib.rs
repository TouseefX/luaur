extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;

extern crate self as luaur_analysis;

pub use luaur_ast::rtti;
pub use luaur_common::{FFlag, FInt};
