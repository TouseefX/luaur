use crate::enums::normalization_result::NormalizationResult;
use crate::records::fuel_initializer::FuelInitializer;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::normalizer_hit_limits::NormalizerHitLimits;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Normalizer {
    pub fn is_inhabited_type_id(&mut self, ty: TypeId) -> NormalizationResult {
        if self.cache_inhabitance {
            if let Some(result) = self.cached_is_inhabited.find(&ty) {
                return if *result {
                    NormalizationResult::True
                } else {
                    NormalizationResult::False
                };
            }
        }

        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null_mut());

        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut fi = FuelInitializer {
                normalizer: self as *mut Normalizer,
                initialized_fuel: false,
            };
            fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer);
            let _fi = fi;

            let result = self.is_inhabited_type_id_set_type_id(ty, &mut seen);

            if self.cache_inhabitance {
                if result == NormalizationResult::True {
                    *self.cached_is_inhabited.get_or_insert(ty) = true;
                } else if result == NormalizationResult::False {
                    *self.cached_is_inhabited.get_or_insert(ty) = false;
                }
            }

            result
        })) {
            Ok(result) => result,
            Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => {
                NormalizationResult::HitLimits
            }
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }
}
