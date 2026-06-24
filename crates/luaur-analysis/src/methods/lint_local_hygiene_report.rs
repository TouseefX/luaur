use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
    pub fn report(&mut self) {
        let locals = self
            .locals
            .iter()
            .map(|(local, info)| (*local, info.clone()))
            .collect::<alloc::vec::Vec<_>>();

        for (local, info) in locals {
            if info.used {
                self.report_used_local(local, &info);
            } else if !info.defined.is_null() {
                self.report_unused_local(local, &info);
            }
        }
    }
}
