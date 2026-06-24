#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum ParenthesesRecommendation {
    None,
    CursorAfter,
    CursorInside,
}

impl Default for ParenthesesRecommendation {
    fn default() -> Self {
        Self::None
    }
}
