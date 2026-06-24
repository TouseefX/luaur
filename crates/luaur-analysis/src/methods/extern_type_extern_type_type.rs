use crate::records::class_user_data::ClassUserData;
use crate::records::extern_type::ExternType;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::props_type_alt_c::Props;
use crate::type_aliases::tags::Tags;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;
use luaur_ast::records::location::Location;

impl ExternType {
    pub fn extern_type_name_props_optional_type_id_optional_type_id_tags_shared_ptr_class_user_data_module_name_optional_location(
        name: Name,
        props: Props,
        parent: Option<TypeId>,
        metatable: Option<TypeId>,
        tags: Tags,
        user_data: Arc<ClassUserData>,
        definition_module_name: ModuleName,
        definition_location: Option<Location>,
    ) -> Self {
        Self {
            name,
            props,
            parent,
            metatable,
            tags,
            user_data: Some(user_data),
            definition_module_name,
            definition_location,
            indexer: None,
            relation: None,
        }
    }
}
