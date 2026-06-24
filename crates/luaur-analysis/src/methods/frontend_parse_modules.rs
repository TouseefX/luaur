use crate::records::frontend::Frontend;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::vec::Vec;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Frontend {
    pub fn parse_modules(&mut self, names: &Vec<ModuleName>) {
        LUAU_TIMETRACE_SCOPE!("Frontend::parseModules", "Frontend");

        let mut seen: DenseHashSet<ModuleName> = DenseHashSet::new(ModuleName::default());

        for name in names {
            if seen.contains(name) {
                continue;
            }

            if let Some(node) = self.source_nodes.get(name) {
                if !node.has_dirty_source_module() {
                    seen.insert(name.clone());
                    continue;
                }
            }

            let mut queue: Vec<ModuleName> = Vec::new();
            // C++ passes a `seenSet` predicate to `parseGraph` to short-circuit
            // already-seen modules during traversal. The Rust `parse_graph` does
            // not expose that callback; the top-level `seen` set below still
            // prevents redundant re-parsing of root modules.
            self.parse_graph(&mut queue, name, &TypeCheckLimits::default(), false);

            seen.insert(name.clone());
        }
    }
}
