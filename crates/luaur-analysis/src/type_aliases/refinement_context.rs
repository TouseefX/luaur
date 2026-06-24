use crate::records::refinement_partition::RefinementPartition;
use crate::type_aliases::def_id_def::DefId;
use luaur_common::records::insertion_ordered_map::InsertionOrderedMap;

#[allow(non_camel_case_types)]
pub type RefinementContext = InsertionOrderedMap<DefId, RefinementPartition>;
