use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::usage_finder::UsageFinder;
use crate::type_aliases::name_type::Name;
use alloc::string::ToString;
use alloc::vec::Vec;

impl UsageFinder {
    pub fn usage_finder(dfg: *mut DataFlowGraph) -> Self {
        // Field initializers: referencedBindings{""}, referencedImportedBindings{{"", ""}}.
        let mut referenced_bindings: Vec<Name> = Vec::new();
        referenced_bindings.push("".to_string());

        let mut referenced_imported_bindings: Vec<(Name, Name)> = Vec::new();
        referenced_imported_bindings.push(("".to_string(), "".to_string()));

        // We explicitly suggest that the usage finder populate types for instance and enum by default
        // These are common enough types that sticking them in the environment is a good idea
        // and it lets magic functions work correctly too.
        referenced_bindings.push("Instance".to_string());
        referenced_bindings.push("Enum".to_string());

        UsageFinder {
            dfg,
            declared_aliases: Default::default(),
            local_bindings_referenced: Vec::new(),
            mentioned_defs: Default::default(),
            referenced_bindings,
            referenced_imported_bindings,
            global_defs_to_pre_populate: Vec::new(),
            global_functions_referenced: Vec::new(),
            symbols_to_refine: Vec::new(),
        }
    }
}
