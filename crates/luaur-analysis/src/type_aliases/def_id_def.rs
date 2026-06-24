//! Source: `Analysis/include/Luau/Def.h:16` (hand-ported)
// C++ `using DefId = NotNull<const Def>` — a never-null borrowed pointer.
// (Previously mis-aliased to ControlFlowGraph.h's SymDef/Definition, which is
// the NEW dataflow system's unrelated DefId.)
#[allow(non_camel_case_types)]
pub type DefId = *const crate::records::def::Def;
