use crate::enums::scope_type::ScopeType;
use crate::type_aliases::bindings::Bindings;
use crate::type_aliases::props_data_flow_graph::Props;

#[derive(Debug, Clone)]
pub struct DfgScope {
    pub(crate) parent: *mut DfgScope,
    pub(crate) scope_type: ScopeType,
    pub(crate) bindings: Bindings,
    pub(crate) props: Props,
}
