use crate::records::alias_cycle_tracker::AliasCycleTracker;
use alloc::string::String;
use luaur_common::functions::format::format;

impl AliasCycleTracker {
    pub fn add(&mut self, alias: String) -> Option<String> {
        if self.seen.contains(&alias) {
            return Some(format(format_args!(
                "detected alias cycle ({})",
                self.get_stringified_cycle(&alias)
            )));
        }

        self.seen.insert(alias.clone());
        self.ordered.push(alias);
        None
    }
}
