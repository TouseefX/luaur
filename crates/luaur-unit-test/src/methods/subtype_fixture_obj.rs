use crate::records::subtype_fixture::SubtypeFixture;
use alloc::string::String;
use luaur_analysis::records::extern_type::ExternType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn obj(&mut self, name: &str, parent: Option<TypeId>) -> TypeId {
        self.arena.add_type(ExternType {
            name: String::from(name),
            props: Default::default(),
            parent: Some(parent.unwrap_or(self.builtin_types.objectType)),
            metatable: None,
            tags: Default::default(),
            user_data: None,
            definition_module_name: String::new(),
            definition_location: None,
            indexer: None,
            relation: None,
        })
    }
}
