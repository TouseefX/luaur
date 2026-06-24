//! Source: `Analysis/src/TypePath.cpp:469-549` (hand-ported)
use crate::enums::type_field::TypeField;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_or_pack_alt_r::get_type_or_pack as get_type_or_pack_ty;
use crate::functions::get_type_or_pack_alt_s::get_type_or_pack_mut_2;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::traversal_state::TraversalState;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;

impl TraversalState {
    pub fn traverse_type_path_type_field(&mut self, field: TypeField) -> bool {
        if self.check_invariants() {
            return false;
        }

        match field {
            TypeField::Table => {
                let mt = unsafe { get_type_or_pack_ty::<MetatableType>(&self.current) };
                if !mt.is_null() {
                    self.update_current_type_id(unsafe { (*mt).table });
                    return true;
                }
                false
            }
            TypeField::Metatable => {
                let current_type = unsafe {
                    crate::functions::get_type_or_pack::get_type_or_pack_mut::<TypeId>(
                        &self.current,
                    )
                };
                if !current_type.is_null() {
                    if let Some(mt) = get_metatable_type_id_not_null_builtin_types(
                        unsafe { *current_type },
                        unsafe { &*self.builtin_types },
                    ) {
                        self.update_current_type_id(mt);
                        return true;
                    }
                }
                false
            }
            TypeField::LowerBound | TypeField::UpperBound => {
                let ft = unsafe { get_type_or_pack_ty::<FreeType>(&self.current) };
                if !ft.is_null() {
                    let target = if field == TypeField::LowerBound {
                        unsafe { (*ft).lower_bound }
                    } else {
                        unsafe { (*ft).upper_bound }
                    };
                    self.update_current_type_id(target);
                    return true;
                }
                false
            }
            TypeField::IndexLookup | TypeField::IndexResult => {
                let mut indexer: *const TableIndexer = core::ptr::null();

                let tt = unsafe { get_type_or_pack_ty::<TableType>(&self.current) };
                if !tt.is_null() && unsafe { (*tt).indexer.is_some() } {
                    indexer = unsafe { (*tt).indexer.as_ref().unwrap() } as *const TableIndexer;
                } else {
                    let mt = unsafe { get_type_or_pack_ty::<MetatableType>(&self.current) };
                    if !mt.is_null() {
                        let mt_tab =
                            unsafe { get_type_id::<TableType>(follow_type_id((*mt).table)) };
                        if !mt_tab.is_null() && unsafe { (*mt_tab).indexer.is_some() } {
                            indexer = unsafe { (*mt_tab).indexer.as_ref().unwrap() }
                                as *const TableIndexer;
                        } else {
                            let mt_mt = unsafe {
                                get_type_id::<TableType>(follow_type_id((*mt).metatable))
                            };
                            if !mt_mt.is_null() && unsafe { (*mt_mt).indexer.is_some() } {
                                indexer = unsafe { (*mt_mt).indexer.as_ref().unwrap() }
                                    as *const TableIndexer;
                            }
                        }
                    } else {
                        // Note: we don't appear to walk the class hierarchy for
                        // indexers
                        let ct = unsafe { get_type_or_pack_ty::<ExternType>(&self.current) };
                        if !ct.is_null() && unsafe { (*ct).indexer.is_some() } {
                            indexer =
                                unsafe { (*ct).indexer.as_ref().unwrap() } as *const TableIndexer;
                        }
                    }
                }

                if !indexer.is_null() {
                    let target = if field == TypeField::IndexLookup {
                        unsafe { (*indexer).index_type }
                    } else {
                        unsafe { (*indexer).index_result_type }
                    };
                    self.update_current_type_id(target);
                    return true;
                }
                false
            }
            TypeField::Negated => {
                let nt = unsafe { get_type_or_pack_ty::<NegationType>(&self.current) };
                if !nt.is_null() {
                    self.update_current_type_id(unsafe { (*nt).ty });
                    return true;
                }
                false
            }
            TypeField::Variadic => {
                let vtp = unsafe { get_type_or_pack_mut_2::<VariadicTypePack>(&self.current) };
                if !vtp.is_null() {
                    self.update_current_type_id(unsafe { (*vtp).ty });
                    return true;
                }
                false
            }
        }
    }
}
