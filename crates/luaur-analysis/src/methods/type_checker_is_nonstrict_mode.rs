use crate::records::type_checker::TypeChecker;
use luaur_ast::enums::mode::Mode;

impl TypeChecker {
    pub fn is_nonstrict_mode(&self) -> bool {
        let module = self.current_module.as_ref().unwrap();
        module.mode == Mode::Nonstrict || module.mode == Mode::NoCheck
    }
}
