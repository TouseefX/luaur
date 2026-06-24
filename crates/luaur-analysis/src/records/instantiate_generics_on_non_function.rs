#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstantiateGenericsOnNonFunction {
    pub(crate) interesting_edge_case: crate::enums::interesting_edge_case::InterestingEdgeCase,
}

impl InstantiateGenericsOnNonFunction {
    pub const None: crate::enums::interesting_edge_case::InterestingEdgeCase =
        crate::enums::interesting_edge_case::InterestingEdgeCase::None;
    pub const MetatableCall: crate::enums::interesting_edge_case::InterestingEdgeCase =
        crate::enums::interesting_edge_case::InterestingEdgeCase::MetatableCall;
    pub const Intersection: crate::enums::interesting_edge_case::InterestingEdgeCase =
        crate::enums::interesting_edge_case::InterestingEdgeCase::Intersection;
}
