use crate::records::normalizer::Normalizer;
use crate::records::normalizer_hit_limits::NormalizerHitLimits;

impl Normalizer {
    pub fn consume_fuel(&mut self) {
        if let Some(fuel) = self.fuel.as_mut() {
            *fuel -= 1;
            if *fuel <= 0 {
                std::panic::resume_unwind(Box::new(NormalizerHitLimits::default()));
            }
        }
    }
}
