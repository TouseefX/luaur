//! Faithful port of `TypeSimplifier::intersectProperty` (Simplify.cpp:1790-1827).
use crate::records::property_type::Property;
use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeSimplifier {
    pub fn intersect_property(
        &self,
        target: &Property,
        discriminant: &Property,
        seen: &mut DenseHashSet<TypeId>,
    ) -> Option<Property> {
        // NOTE: I invite the reader to refactor the below code as a fun coding
        // exercise. It looks ugly to me, but I don't think we can make it
        // any cleaner.

        let mut prop = Property::default();
        prop.deprecated = target.deprecated || discriminant.deprecated;

        // We're trying to follow the following rules for both read and write types:
        // * If the type is present on both properties, intersect it, and return
        //   `None` if we fail.
        // * If the type only exists on one property or the other, take that.

        if target.read_ty.is_some() && discriminant.read_ty.is_some() {
            prop.read_ty = self
                .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
                    target.read_ty.unwrap(),
                    discriminant.read_ty.unwrap(),
                    seen,
                );
            if prop.read_ty.is_none() {
                return None;
            }
        } else if target.read_ty.is_some() && discriminant.read_ty.is_none() {
            prop.read_ty = target.read_ty;
        } else if target.read_ty.is_none() && discriminant.read_ty.is_some() {
            prop.read_ty = discriminant.read_ty;
        }

        if target.write_ty.is_some() && discriminant.write_ty.is_some() {
            prop.write_ty = self
                .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
                    target.write_ty.unwrap(),
                    discriminant.write_ty.unwrap(),
                    seen,
                );
            if prop.write_ty.is_none() {
                return None;
            }
        } else if target.write_ty.is_some() && discriminant.write_ty.is_none() {
            prop.write_ty = target.write_ty;
        } else if target.write_ty.is_none() && discriminant.write_ty.is_some() {
            prop.write_ty = discriminant.write_ty;
        }

        Some(prop)
    }
}
