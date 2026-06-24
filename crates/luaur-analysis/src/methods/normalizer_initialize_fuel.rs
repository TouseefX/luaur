use crate::records::normalizer::Normalizer;
use luaur_common::FInt;

impl Normalizer {
    pub fn initialize_fuel(&mut self) -> bool {
        if self.fuel.is_some() {
            return false;
        }

        self.fuel = Some(luaur_common::FInt::LuauNormalizerInitialFuel.get());
        true
    }
}
