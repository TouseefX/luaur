use alloc::sync::Arc;
use luaur_ast::records::location::Location;

use crate::records::class_user_data::ClassUserData;
use crate::records::table_indexer::TableIndexer;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::nominal_relation::NominalRelation;
use crate::type_aliases::props_type_alt_c::Props;
use crate::type_aliases::tags::Tags;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct ExternType {
    pub name: Name,
    pub props: Props,
    pub parent: Option<TypeId>,
    pub metatable: Option<TypeId>,
    pub tags: Tags,
    pub user_data: Option<Arc<ClassUserData>>,
    pub definition_module_name: ModuleName,
    pub definition_location: Option<Location>,
    pub indexer: Option<TableIndexer>,
    /// This field represents a bidirectional relationship between classes and object types
    /// Given a Class, this relation should be a Obj in the variant, representing an instantiation of the class
    /// Given a Object, this relation should be a Klass in the variant, representing the class prototype
    /// Other sources of Extern Types will not have this relation set - this is for the classes fixture so that
    /// we can go between class and object easily, given just the extern type
    pub relation: Option<NominalRelation>,
}
