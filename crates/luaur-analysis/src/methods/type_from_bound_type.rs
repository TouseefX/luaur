use crate::records::r#type::Type;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_variant::TypeVariant;

impl From<BoundType> for Type {
    fn from(bound: BoundType) -> Self {
        Type::new(TypeVariant::Bound(bound.boundTo))
    }
}
