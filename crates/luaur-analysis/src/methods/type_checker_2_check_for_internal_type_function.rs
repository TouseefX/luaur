use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::internal_type_function_finder::InternalTypeFunctionFinder;
use crate::records::pack_where_clause_needed::PackWhereClauseNeeded;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::where_clause_needed::WhereClauseNeeded;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn check_for_internal_type_function(&mut self, ty: TypeId, location: Location) {
        let mut finder = InternalTypeFunctionFinder::internal_type_function_finder(
            &mut self.function_decl_stack,
        );
        finder.traverse_type_id(ty);

        for internal in finder.internal_functions.iter() {
            if self.should_suppress_uninhabited_type_function_error(*internal) {
                continue;
            }

            self.report_error_type_error_data_location(
                WhereClauseNeeded { ty: *internal }.into(),
                &location,
            );
        }

        for internal in finder.internal_pack_functions.iter() {
            self.report_error_type_error_data_location(
                PackWhereClauseNeeded { tp: *internal }.into(),
                &location,
            );
        }
    }
}
