use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::substitution::Substitution;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Substitution {
    pub fn replace_children_type_id(&mut self, ty: TypeId) {
        unsafe {
            LUAU_ASSERT!(ty == (*self.base.log).follow_type_id(ty));
        }

        if self.base.ignore_children_type_id(ty) {
            return;
        }

        if unsafe { (*ty).owning_arena != self.arena } {
            return;
        }

        if !unsafe { get_mutable_type_id::<FunctionType>(ty) }.is_null() {
            let ftv = unsafe { &mut *get_mutable_type_id::<FunctionType>(ty) };
            for generic in ftv.generics.iter_mut() {
                *generic = self.replace_type_id(*generic);
            }
            for generic_pack in ftv.generic_packs.iter_mut() {
                *generic_pack = self.replace_type_pack_id(*generic_pack);
            }
            ftv.arg_types = self.replace_type_pack_id(ftv.arg_types);
            ftv.ret_types = self.replace_type_pack_id(ftv.ret_types);
        } else if !unsafe { get_mutable_type_id::<TableType>(ty) }.is_null() {
            let ttv = unsafe { &mut *get_mutable_type_id::<TableType>(ty) };
            LUAU_ASSERT!(ttv.bound_to.is_none());
            for (_name, prop) in ttv.props.iter_mut() {
                if prop.read_ty.is_some() {
                    prop.read_ty = Some(self.replace_type_id(prop.read_ty.unwrap()));
                }
                if prop.write_ty.is_some() {
                    prop.write_ty = Some(self.replace_type_id(prop.write_ty.unwrap()));
                }
            }
            if let Some(ref mut indexer) = ttv.indexer {
                indexer.index_type = self.replace_type_id(indexer.index_type);
                indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
            }
            for itp in ttv.instantiated_type_params.iter_mut() {
                *itp = self.replace_type_id(*itp);
            }
            for itp in ttv.instantiated_type_pack_params.iter_mut() {
                *itp = self.replace_type_pack_id(*itp);
            }
        } else if !unsafe { get_mutable_type_id::<MetatableType>(ty) }.is_null() {
            let mtv = unsafe { &mut *get_mutable_type_id::<MetatableType>(ty) };
            mtv.table = self.replace_type_id(mtv.table);
            mtv.metatable = self.replace_type_id(mtv.metatable);
        } else if !unsafe { get_mutable_type_id::<UnionType>(ty) }.is_null() {
            let utv = unsafe { &mut *get_mutable_type_id::<UnionType>(ty) };
            for opt in utv.options.iter_mut() {
                *opt = self.replace_type_id(*opt);
            }
        } else if !unsafe { get_mutable_type_id::<IntersectionType>(ty) }.is_null() {
            let itv = unsafe { &mut *get_mutable_type_id::<IntersectionType>(ty) };
            for part in itv.parts.iter_mut() {
                *part = self.replace_type_id(*part);
            }
        } else if !unsafe { get_mutable_type_id::<PendingExpansionType>(ty) }.is_null() {
            let petv = unsafe { &mut *get_mutable_type_id::<PendingExpansionType>(ty) };
            for a in petv.type_arguments.iter_mut() {
                *a = self.replace_type_id(*a);
            }
            for a in petv.pack_arguments.iter_mut() {
                *a = self.replace_type_pack_id(*a);
            }
        } else if !unsafe { get_mutable_type_id::<TypeFunctionInstanceType>(ty) }.is_null() {
            let tfit = unsafe { &mut *get_mutable_type_id::<TypeFunctionInstanceType>(ty) };
            for a in tfit.type_arguments.iter_mut() {
                *a = self.replace_type_id(*a);
            }
            for a in tfit.pack_arguments.iter_mut() {
                *a = self.replace_type_pack_id(*a);
            }
        } else if !unsafe { get_mutable_type_id::<ExternType>(ty) }.is_null() {
            let etv = unsafe { &mut *get_mutable_type_id::<ExternType>(ty) };
            for (_name, prop) in etv.props.iter_mut() {
                if prop.read_ty.is_some() {
                    prop.read_ty = Some(self.replace_type_id(prop.read_ty.unwrap()));
                }
                if prop.write_ty.is_some() {
                    prop.write_ty = Some(self.replace_type_id(prop.write_ty.unwrap()));
                }
            }
            if let Some(ref mut parent) = etv.parent {
                *parent = self.replace_type_id(*parent);
            }
            if let Some(ref mut metatable) = etv.metatable {
                *metatable = self.replace_type_id(*metatable);
            }
            if let Some(ref mut indexer) = etv.indexer {
                indexer.index_type = self.replace_type_id(indexer.index_type);
                indexer.index_result_type = self.replace_type_id(indexer.index_result_type);
            }
        } else if !unsafe { get_mutable_type_id::<NegationType>(ty) }.is_null() {
            let ntv = unsafe { &mut *get_mutable_type_id::<NegationType>(ty) };
            ntv.ty = self.replace_type_id(ntv.ty);
        }
    }
}
