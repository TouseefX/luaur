use crate::records::type_error::TypeError;
use luaur_ast::records::location::Location;

impl TypeError {
    pub fn type_error() -> Self {
        Self {
            location: Location::default(),
            module_name: alloc::string::String::new(),
            data: unsafe { core::mem::zeroed() },
        }
    }
}
