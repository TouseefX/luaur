use crate::records::demoter::Demoter;
use crate::records::type_level::TypeLevel;

impl Demoter {
    pub fn demoted_level(&mut self, level: TypeLevel) -> TypeLevel {
        TypeLevel {
            level: level.level + 5000,
            subLevel: level.subLevel,
        }
    }
}
