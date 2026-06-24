use crate::records::frontend_module_resolver::FrontendModuleResolver;
use crate::records::source_node::SourceNode;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn accumulate_errors(
    source_nodes: &BTreeMap<ModuleName, Arc<SourceNode>>,
    module_resolver: &FrontendModuleResolver,
    name: &ModuleName,
) -> ErrorVec {
    let mut seen: BTreeSet<ModuleName> = BTreeSet::new();
    let mut queue: Vec<ModuleName> = vec![name.clone()];
    let mut result = ErrorVec::new();

    while let Some(next) = queue.pop() {
        if seen.contains(&next) {
            continue;
        }
        seen.insert(next.clone());

        let Some(source_node) = source_nodes.get(&next) else {
            continue;
        };

        for dependency in source_node.require_set.iter() {
            queue.push(dependency.clone());
        }

        let module = {
            let _lock = module_resolver.module_mutex.lock().unwrap();
            module_resolver.modules.get(&next).cloned()
        };
        let Some(module) = module else {
            continue;
        };

        let prev_size = result.len();
        result.extend(module.errors.iter().rev().cloned());
        result[prev_size..].sort_by(|left, right| right.location.begin.cmp(&left.location.begin));
    }

    result.reverse();
    result
}
