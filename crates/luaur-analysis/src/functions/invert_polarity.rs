use crate::enums::polarity::Polarity;

pub fn invert(p: Polarity) -> Polarity {
    match p {
        Polarity::Positive => Polarity::Negative,
        Polarity::Negative => Polarity::Positive,
        _ => p,
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use invert as invert_polarity;
