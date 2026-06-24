use crate::records::lint_duplicate_function::LintDuplicateFunction;
use alloc::string::String;
use luaur_ast::records::location::Location;

impl LintDuplicateFunction {
    pub fn track_function(&mut self, location: Location, name: &str) {
        if name.is_empty() {
            return;
        }

        let mut other_location = None;
        {
            let defn = self.defns.get_or_insert(String::from(name));
            if defn.end.line == 0 && defn.end.column == 0 {
                *defn = location;
            } else {
                other_location = Some(*defn);
            }
        }

        if let Some(defn) = other_location {
            self.report_location_c_char_location(name, location, defn);
        }
    }
}
