use crate::records::builtin_types::BuiltinTypes;
use crate::records::replace_generics::ReplaceGenerics;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::type_level::TypeLevel;

#[derive(Debug, Clone)]
pub struct Instantiation {
    pub base: Substitution,
    pub builtin_types: *mut BuiltinTypes,
    pub level: TypeLevel,
    pub scope: *mut Scope,
    pub reusable_replace_generics: ReplaceGenerics,
}
