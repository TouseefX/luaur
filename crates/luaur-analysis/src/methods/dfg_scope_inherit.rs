use crate::records::def::Def;
use crate::records::dfg_scope::DfgScope;
use crate::records::symbol::Symbol;
use alloc::string::String;

impl DfgScope {
    pub fn inherit(&mut self, child_scope: *const DfgScope) {
        // C++:
        //   for (const auto& [k, a] : childScope->bindings)
        //       if (lookup(k)) bindings[k] = a;
        //   for (const auto& [k1, a1] : childScope->props)
        //       for (const auto& [k2, a2] : a1)
        //           props[k1][k2] = a2;
        unsafe {
            let child_bindings: alloc::vec::Vec<(Symbol, *const Def)> = (*child_scope)
                .bindings
                .iter()
                .map(|(k, a)| (k.clone(), *a))
                .collect();
            for (k, a) in child_bindings {
                if self.lookup_symbol(k.clone()).is_some() {
                    *self.bindings.get_or_insert(k) = a;
                }
            }

            let child_props: alloc::vec::Vec<(*const Def, alloc::vec::Vec<(String, *const Def)>)> =
                (*child_scope)
                    .props
                    .iter()
                    .map(|(k1, a1)| (*k1, a1.iter().map(|(k2, a2)| (k2.clone(), *a2)).collect()))
                    .collect();
            for (k1, a1) in child_props {
                let entry = self.props.get_or_insert(k1);
                for (k2, a2) in a1 {
                    entry.insert(k2, a2);
                }
            }
        }
    }
}
