use crate::records::fuel_initializer::FuelInitializer;

impl FuelInitializer {
    pub fn operator_assign(&mut self, _rhs: &FuelInitializer) -> &mut FuelInitializer {
        // deleted assignment operator
        self
    }
}
