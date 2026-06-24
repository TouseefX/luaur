use crate::enums::polarity::Polarity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterState {
    pub count: usize,
    pub polarity: Polarity,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            count: 0,
            polarity: Polarity::None,
        }
    }
}
