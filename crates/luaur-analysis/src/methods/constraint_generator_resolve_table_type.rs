use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::polarity_of_access::polarity_of_access;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generic_error::GenericError;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::enums::ast_table_access::AstTableAccess;
use luaur_ast::records::ast_table_indexer::AstTableIndexer;
use luaur_ast::records::ast_table_prop::AstTableProp;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::location::Location;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn resolve_table_type(
        &mut self,
        scope: *mut Scope,
        _ty: *mut AstType,
        tab: *mut AstTypeTable,
        in_type_arguments: bool,
        _replace_error_with_fresh: bool,
    ) -> TypeId {
        let mut props: alloc::collections::BTreeMap<
            crate::type_aliases::name_type::Name,
            Property,
        > = alloc::collections::BTreeMap::new();
        let mut indexer: Option<TableIndexer> = None;

        let p = self.polarity;

        // SAFETY: tab is a valid pointer to AstTypeTable
        let tab_ref = unsafe { &*tab };
        let props_array = &tab_ref.props;

        for i in 0..props_array.size as usize {
            let prop_ref = unsafe { &*props_array.data.add(i) };
            let name: alloc::string::String = unsafe {
                core::ffi::CStr::from_ptr(prop_ref.name.value)
                    .to_string_lossy()
                    .into_owned()
            };
            let prop_access = prop_ref.access;

            // Set the polarity for the inner type
            self.polarity = polarity_of_access(prop_access, p);
            let cur_polarity = self.polarity;

            let prop_ty = self.resolve_type(
                scope,
                prop_ref.r#type as *mut AstType,
                in_type_arguments,
                false,
                cur_polarity,
            );

            let prop_ref_mut = props.entry(name).or_insert_with(Property::default);

            prop_ref_mut.type_location = Some(prop_ref.location);

            match prop_access {
                AstTableAccess::ReadWrite => {
                    prop_ref_mut.read_ty = Some(prop_ty);
                    prop_ref_mut.write_ty = Some(prop_ty);
                }
                AstTableAccess::Read => {
                    prop_ref_mut.read_ty = Some(prop_ty);
                }
                AstTableAccess::Write => {
                    prop_ref_mut.write_ty = Some(prop_ty);
                }
                _ => {
                    // C++ calls ice->ice(...) for unexpected access; faithfully left as unreachable
                }
            }
        }

        if !tab_ref.indexer.is_null() {
            let ast_indexer = unsafe { &*tab_ref.indexer };
            let indexer_access = ast_indexer.access;

            if indexer_access == AstTableAccess::Read {
                if !FFlag::LuauReadOnlyIndexers.get() {
                    self.report_error(
                        ast_indexer.access_location.unwrap_or(Location::new(
                            ast_indexer.location.begin,
                            ast_indexer.location.begin,
                        )),
                        TypeErrorData::GenericError(GenericError::new(
                            "read keyword is illegal here".to_string(),
                        )),
                    );
                } else {
                    self.polarity = p;
                    let cur_polarity = self.polarity;
                    let index_ty = self.resolve_type(
                        scope,
                        ast_indexer.index_type as *mut AstType,
                        in_type_arguments,
                        false,
                        cur_polarity,
                    );
                    let cur_polarity2 = self.polarity;
                    let result_ty = self.resolve_type(
                        scope,
                        ast_indexer.result_type as *mut AstType,
                        in_type_arguments,
                        false,
                        cur_polarity2,
                    );
                    indexer = Some(TableIndexer {
                        index_type: index_ty,
                        index_result_type: result_ty,
                        is_read_only: true,
                    });
                }
            } else if indexer_access == AstTableAccess::Write {
                self.report_error(
                    ast_indexer.access_location.unwrap_or(Location::new(
                        ast_indexer.location.begin,
                        ast_indexer.location.begin,
                    )),
                    TypeErrorData::GenericError(GenericError::new(
                        "write keyword is illegal here".to_string(),
                    )),
                );
            } else if indexer_access == AstTableAccess::ReadWrite {
                self.polarity = Polarity::Mixed;
                let cur_polarity = self.polarity;
                let index_ty = self.resolve_type(
                    scope,
                    ast_indexer.index_type as *mut AstType,
                    in_type_arguments,
                    false,
                    cur_polarity,
                );
                let cur_polarity2 = self.polarity;
                let result_ty = self.resolve_type(
                    scope,
                    ast_indexer.result_type as *mut AstType,
                    in_type_arguments,
                    false,
                    cur_polarity2,
                );
                indexer = Some(TableIndexer {
                    index_type: index_ty,
                    index_result_type: result_ty,
                    is_read_only: false,
                });
            }
            // else: Unexpected property access - handled by C++ ice
        }

        self.polarity = p;

        let table_ty = unsafe {
            let tt =
                TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                    &props,
                    indexer,
                    (*scope).level,
                    scope,
                    TableState::Sealed,
                );
            let type_ptr = (*self.arena).add_type(tt);
            type_ptr
        };

        // SAFETY: table_ty is a valid TypeId returned by arena.add_type
        let ttv = unsafe { get_mutable_type_id::<TableType>(table_ty) };

        // SAFETY: ttv is a valid mutable pointer to TableType
        unsafe {
            (*ttv).definition_module_name = self.module.as_ref().unwrap().name.clone();
            (*ttv).definition_location = (*tab).base.base.location;
        }

        table_ty
    }
}
