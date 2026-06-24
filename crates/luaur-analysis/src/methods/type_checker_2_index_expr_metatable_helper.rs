use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn type_checker_2_index_expr_metatable_helper(
        &mut self,
        index_expr: *mut AstExprIndexExpr,
        meta_table: *const MetatableType,
        expr_type: TypeId,
        index_type: TypeId,
    ) {
        let table_followed = unsafe { follow_type_id(unsafe { (*meta_table).table() }) };
        let table_ptr = unsafe { get_type_id::<TableType>(table_followed) };
        if !table_ptr.is_null() {
            let tt = unsafe { &*table_ptr };
            if let Some(ref indexer) = tt.indexer {
                self.test_is_subtype_type_id_type_id_location(
                    index_type,
                    indexer.index_type,
                    unsafe { (*index_expr).base.base.location },
                );
                return;
            }
        }

        let mt_ptr = unsafe { get_type_id::<MetatableType>(table_followed) };
        if !mt_ptr.is_null() {
            self.type_checker_2_index_expr_metatable_helper(
                index_expr,
                unsafe { &*mt_ptr },
                expr_type,
                index_type,
            );
            return;
        }

        let metatable_followed = unsafe { follow_type_id(unsafe { (*meta_table).metatable() }) };
        let tmt_ptr = unsafe { get_type_id::<TableType>(metatable_followed) };
        if !tmt_ptr.is_null() {
            let tmt = unsafe { &*tmt_ptr };
            if let Some(ref indexer) = tmt.indexer {
                self.test_is_subtype_type_id_type_id_location(
                    index_type,
                    indexer.index_type,
                    unsafe { (*index_expr).base.base.location },
                );
                return;
            }
        }

        let mtmt_ptr = unsafe { get_type_id::<MetatableType>(metatable_followed) };
        if !mtmt_ptr.is_null() {
            self.type_checker_2_index_expr_metatable_helper(
                index_expr,
                unsafe { &*mtmt_ptr },
                expr_type,
                index_type,
            );
            return;
        }

        let cannot_extend = CannotExtendTable {
            table_type: expr_type,
            context: crate::records::cannot_extend_table::Context::Indexer,
            prop: "indexer??".to_string(),
        };
        self.report_error_type_error_data_location(
            crate::records::type_error_data::TypeErrorData::CannotExtendTable(cannot_extend),
            &unsafe { (*index_expr).base.base.location },
        );
    }
}
