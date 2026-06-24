use crate::records::builtin_type_functions::BuiltinTypeFunctions;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::boxed::Box;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct BuiltinTypes {
    pub(crate) arena: Box<TypeArena>,
    pub(crate) debugFreezeArena: bool,
    pub typeFunctions: Box<BuiltinTypeFunctions>,
    pub nilType: TypeId,
    pub numberType: TypeId,
    pub integerType: TypeId,
    pub stringType: TypeId,
    pub booleanType: TypeId,
    pub threadType: TypeId,
    pub bufferType: TypeId,
    pub functionType: TypeId,
    pub externType: TypeId,
    pub objectType: TypeId,
    pub classType: TypeId,
    pub tableType: TypeId,
    pub emptyTableType: TypeId,
    pub trueType: TypeId,
    pub falseType: TypeId,
    pub anyType: TypeId,
    pub unknownType: TypeId,
    pub neverType: TypeId,
    pub errorType: TypeId,
    pub noRefineType: TypeId,
    pub falsyType: TypeId,
    pub truthyType: TypeId,
    pub notNilType: TypeId,
    pub optionalNumberType: TypeId,
    pub optionalStringType: TypeId,
    pub emptyTypePack: TypePackId,
    pub anyTypePack: TypePackId,
    pub unknownTypePack: TypePackId,
    pub neverTypePack: TypePackId,
    pub uninhabitableTypePack: TypePackId,
    pub errorTypePack: TypePackId,
}

impl Clone for BuiltinTypes {
    fn clone(&self) -> Self {
        panic!("BuiltinTypes is not cloneable in C++ (copy constructor is deleted)");
    }
}
