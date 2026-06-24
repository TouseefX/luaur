use crate::records::fuel_initializer::FuelInitializer;
use crate::records::normalizer::Normalizer;

impl FuelInitializer {
    pub fn fuel_initializer_not_null_normalizer(&mut self, normalizer: *mut Normalizer) {
        self.normalizer = normalizer;
        self.initialized_fuel = unsafe { (*normalizer).initialize_fuel() };
    }
}
