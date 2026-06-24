use crate::records::type_level::TypeLevel;

impl TypeLevel {
    pub fn subsumes(&self, rhs: &TypeLevel) -> bool {
        if self.level < rhs.level {
            return true;
        }
        if self.level > rhs.level {
            return false;
        }
        if self.subLevel == rhs.subLevel {
            return true; // if level == rhs.level and subLevel == rhs.subLevel, then they are the exact same TypeLevel
        }

        // Sibling TypeLevels (that is, TypeLevels that share a level but have a different subLevel) are not considered to subsume one another
        false
    }
}
