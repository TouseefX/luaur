use crate::records::annotation_types_at_location::AnnotationTypesAtLocation;
use crate::records::error_snapshot::ErrorSnapshot;
use crate::records::expr_types_at_location::ExprTypesAtLocation;

#[derive(Debug, Clone, Default)]
pub struct ConstraintGenerationLog {
    pub source: alloc::string::String,
    pub errors: alloc::vec::Vec<ErrorSnapshot>,
    pub expr_type_locations: alloc::vec::Vec<ExprTypesAtLocation>,
    pub annotation_type_locations: alloc::vec::Vec<AnnotationTypesAtLocation>,
}
