use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::table_type::TableType;
use crate::records::tarjan::Tarjan;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::type_aliases::nominal_relation::NominalRelation;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl Tarjan {
    pub fn visit_children_type_id_i32(&mut self, ty: TypeId, _index: i32) {
        let mut ty = ty;
        unsafe {
            LUAU_ASSERT!(ty == (*self.log).follow_type_id(ty));
        }

        if self.ignore_children_visit_type_id(ty) {
            return;
        }

        let pty = unsafe { (*self.log).pending_type_id(ty) };
        if !pty.is_null() {
            ty = unsafe { &(*pty).pending as *const crate::records::r#type::Type };
        }

        let ftv = unsafe { get_type_id::<FunctionType>(ty) };
        if !ftv.is_null() {
            let ftv = unsafe { &*ftv };
            for generic in ftv.generics.iter() {
                self.visit_child_type_id(*generic);
            }
            for generic_pack in ftv.generic_packs.iter() {
                self.visit_child_type_pack_id(*generic_pack);
            }

            self.visit_child_type_pack_id(ftv.arg_types);
            self.visit_child_type_pack_id(ftv.ret_types);
            return;
        }

        let ttv = unsafe { get_type_id::<TableType>(ty) };
        if !ttv.is_null() {
            let ttv = unsafe { &*ttv };
            LUAU_ASSERT!(ttv.bound_to.is_none());
            for (_name, prop) in ttv.props.iter() {
                self.visit_child_optional_ty(prop.read_ty);
                self.visit_child_optional_ty(prop.write_ty);
            }

            if let Some(ref indexer) = ttv.indexer {
                self.visit_child_type_id(indexer.index_type);
                self.visit_child_type_id(indexer.index_result_type);
            }

            for itp in ttv.instantiated_type_params.iter() {
                self.visit_child_type_id(*itp);
            }

            for itp in ttv.instantiated_type_pack_params.iter() {
                self.visit_child_type_pack_id(*itp);
            }
            return;
        }

        let mtv = unsafe { get_type_id::<MetatableType>(ty) };
        if !mtv.is_null() {
            let mtv = unsafe { &*mtv };
            self.visit_child_type_id(mtv.table);
            self.visit_child_type_id(mtv.metatable);
            return;
        }

        let utv = unsafe { get_type_id::<UnionType>(ty) };
        if !utv.is_null() {
            let utv = unsafe { &*utv };
            for opt in utv.options.iter() {
                self.visit_child_type_id(*opt);
            }
            return;
        }

        let itv = unsafe { get_type_id::<IntersectionType>(ty) };
        if !itv.is_null() {
            let itv = unsafe { &*itv };
            for part in itv.parts.iter() {
                self.visit_child_type_id(*part);
            }
            return;
        }

        let petv = unsafe { get_type_id::<PendingExpansionType>(ty) };
        if !petv.is_null() {
            let petv = unsafe { &*petv };
            for a in petv.type_arguments.iter() {
                self.visit_child_type_id(*a);
            }
            for a in petv.pack_arguments.iter() {
                self.visit_child_type_pack_id(*a);
            }
            return;
        }

        let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(ty) };
        if !tfit.is_null() {
            let tfit = unsafe { &*tfit };
            for a in tfit.type_arguments.iter() {
                self.visit_child_type_id(*a);
            }
            for a in tfit.pack_arguments.iter() {
                self.visit_child_type_pack_id(*a);
            }
            return;
        }

        let etv = unsafe { get_type_id::<ExternType>(ty) };
        if !etv.is_null() {
            let etv = unsafe { &*etv };
            for (_name, prop) in etv.props.iter() {
                if prop.read_ty.is_some() {
                    self.visit_child_optional_ty(prop.read_ty);
                }
                if prop.write_ty.is_some() {
                    self.visit_child_optional_ty(prop.write_ty);
                }
            }

            if let Some(parent) = etv.parent {
                self.visit_child_type_id(parent);
            }

            if let Some(metatable) = etv.metatable {
                self.visit_child_type_id(metatable);
            }

            if let Some(ref indexer) = etv.indexer {
                self.visit_child_type_id(indexer.index_type);
                self.visit_child_type_id(indexer.index_result_type);
            }

            if FFlag::DebugLuauUserDefinedClasses.get() {
                if let Some(ref relation) = etv.relation {
                    match relation {
                        NominalRelation::V0(obj) => {
                            self.visit_child_type_id(obj.ty);
                        }
                        NominalRelation::V1(klass) => {
                            self.visit_child_type_id(klass.ty);
                        }
                    }
                }
            }
            return;
        }

        let ntv = unsafe { get_type_id::<NegationType>(ty) };
        if !ntv.is_null() {
            let ntv = unsafe { &*ntv };
            self.visit_child_type_id(ntv.ty);
        }
    }
}
