use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::subtyping_reasoning_hash::SubtypingReasoningHash;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub type SubtypingReasonings = DenseHashSet<SubtypingReasoning, SubtypingReasoningHash>;
