use crate::records::generic_pack_mapping::GenericPackMapping;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_pack_id::TypePackId;

impl PathBuilder {
    /// `components.emplace_back(GenericPackMapping{mappedType})`.
    /// Reference: `TypePath.cpp:269-272`.
    pub fn mapped_generic_pack(&mut self, mapped_type: TypePackId) -> &mut Self {
        self.components
            .push(Component::GenericPackMapping(GenericPackMapping {
                mappedType: mapped_type,
            }));
        self
    }
}
