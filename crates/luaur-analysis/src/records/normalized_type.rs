use crate::records::builtin_types::BuiltinTypes;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::normalized_tyvars::NormalizedTyvars;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct NormalizedType {
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) tops: TypeId,
    pub(crate) booleans: TypeId,
    pub(crate) extern_types: NormalizedExternType,
    pub(crate) errors: TypeId,
    pub(crate) nils: TypeId,
    pub(crate) numbers: TypeId,
    pub(crate) integers: TypeId,
    pub(crate) strings: NormalizedStringType,
    pub(crate) threads: TypeId,
    pub(crate) buffers: TypeId,
    pub(crate) tables: TypeIds,
    pub(crate) functions: NormalizedFunctionType,
    pub(crate) tyvars: NormalizedTyvars,
    pub(crate) is_cacheable: bool,
}
