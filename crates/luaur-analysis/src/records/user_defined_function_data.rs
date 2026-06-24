use crate::records::module::Module;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::name_type_infer::Name;
use alloc::string::String;
use alloc::sync::Weak;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct User_Defined_Function_Data {
    /// Store a weak module reference to ensure the lifetime requirements are preserved
    pub(crate) owner: Weak<Module>,

    /// References to AST elements are owned by the Module allocator which also stores this type
    pub(crate) definition: *mut AstStatTypeFunction,

    pub(crate) environment_function: DenseHashMap<Name, (*mut AstStatTypeFunction, usize)>,
    pub(crate) environment_alias: DenseHashMap<Name, (*mut TypeFun, usize)>,
}

#[allow(non_upper_case_globals)]
impl User_Defined_Function_Data {
    pub(crate) fn new(owner: Weak<Module>) -> Self {
        Self {
            owner,
            definition: core::ptr::null_mut(),
            environment_function: DenseHashMap::new(String::new()),
            environment_alias: DenseHashMap::new(String::new()),
        }
    }

    /// Creates an empty instance with no owning module (used when no owner is present).
    pub(crate) fn new_empty() -> Self {
        Self {
            owner: Weak::new(),
            definition: core::ptr::null_mut(),
            environment_function: DenseHashMap::new(String::new()),
            environment_alias: DenseHashMap::new(String::new()),
        }
    }
}

#[allow(non_camel_case_types)]
pub type UserDefinedFunctionData = User_Defined_Function_Data;
