use crate::records::const_prop_state::ConstPropState;

impl ConstPropState {
    pub fn is_valid_double_for_immediate(&mut self, d: f64) -> bool {
        d >= -4095.0 && d <= 4095.0 && (d as i32) as f64 == d
    }
}
