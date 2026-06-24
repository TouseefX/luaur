#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NormalizerHitLimits;

impl core::fmt::Display for NormalizerHitLimits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Normalizer hit limits")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NormalizerHitLimits {}

unsafe impl Send for NormalizerHitLimits {}
unsafe impl Sync for NormalizerHitLimits {}
