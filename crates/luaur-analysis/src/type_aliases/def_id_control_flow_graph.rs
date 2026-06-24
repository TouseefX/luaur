// C++ (ControlFlowGraph): `using DefId = NotNull<Definition>;` where
// `Definition = SymDef`. NotNull -> raw pointer.
#[allow(non_camel_case_types)]
pub type DefId = *mut crate::type_aliases::definition::Definition;
