//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ControlFlowGraph.h:38:instr_id`
//! Source: `Analysis/include/Luau/ControlFlowGraph.h:38` (hand-ported)
// C++ `using InstrId = NotNull<Instruction>` over ControlFlowGraph.h's
// OWN Instruction variant (previously mis-aliased to Instruction.h's unrelated
// InstrId).
#[allow(non_camel_case_types)]
pub type InstrId = *mut crate::type_aliases::instruction::Instruction;
