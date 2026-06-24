use crate::records::builtin_types::BuiltinTypes;
use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::type_ids::TypeIds;

pub fn reset_to_top(builtin_types: &BuiltinTypes, extern_types: &mut NormalizedExternType) {
    extern_types.ordering.clear();
    extern_types.extern_types.clear();
    extern_types.push_pair(builtin_types.externType, TypeIds::type_ids());
}
