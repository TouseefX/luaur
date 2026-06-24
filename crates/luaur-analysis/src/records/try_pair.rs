#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TryPair<A, B> {
    pub first: A,
    pub second: B,
}

impl<A, B> TryPair<A, B>
where
    A: Copy + Into<bool>,
    B: Copy + Into<bool>,
{
    pub fn as_bool(&self) -> bool {
        self.first.into() && self.second.into()
    }
}

impl<A, B> From<TryPair<A, B>> for bool
where
    A: Into<bool>,
    B: Into<bool>,
{
    fn from(pair: TryPair<A, B>) -> bool {
        pair.first.into() && pair.second.into()
    }
}
