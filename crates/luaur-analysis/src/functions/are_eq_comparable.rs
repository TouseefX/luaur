use crate::enums::normalization_result::NormalizationResult;
use crate::functions::follow_type::follow_type_id;
use crate::records::free_type::FreeType;
use crate::records::intersection_type::IntersectionType;
use crate::records::normalizer::Normalizer;
use crate::records::r#type::Type;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;

/// Rust translation of Luau.Analysis::Analysis::TypeInfer.cpp:are_eq_comparable
pub fn are_eq_comparable(
    arena: &mut TypeArena,
    normalizer: &mut Normalizer,
    a: TypeId,
    b: TypeId,
) -> Option<bool> {
    let a = unsafe { follow_type_id(a) };
    let b = unsafe { follow_type_id(b) };

    let is_exempt = |t: TypeId| -> bool {
        is_nil(t) || unsafe { get_type_id::<FreeType>(t) }.is_null() == false
    };

    if is_exempt(a) || is_exempt(b) {
        return Some(true);
    }

    let c = arena.add_type(Type::new(TypeVariant::Intersection(IntersectionType {
        parts: alloc::vec![a, b],
    })));

    let n = normalizer.normalize(c);

    let nr = normalizer.is_inhabited_normalized_type(n.as_ref());

    match nr {
        NormalizationResult::HitLimits => None,
        NormalizationResult::False => Some(false),
        NormalizationResult::True => Some(true),
    }
}
