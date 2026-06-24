//! Source: `Analysis/src/Unifier2.cpp:407-438` — `Unifier2::unify_(TypeId, const FunctionType*)`.

use crate::enums::unify_result::UnifyResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;

impl Unifier2 {
    pub fn unify_type_id_function_type(
        &mut self,
        sub_ty: TypeId,
        super_fn: &FunctionType,
    ) -> UnifyResult {
        let sub_fn = unsafe { get_type_id::<FunctionType>(sub_ty) };
        let sub_fn = unsafe { &*sub_fn };

        let should_instantiate = (super_fn.generics.is_empty() && !sub_fn.generics.is_empty())
            || (super_fn.generic_packs.is_empty() && !sub_fn.generic_packs.is_empty());

        if should_instantiate {
            for &generic in sub_fn.generics.iter() {
                let generic = unsafe { follow_type_id(generic) };
                let gen = unsafe { get_type_id::<GenericType>(generic) };
                if !gen.is_null() {
                    let polarity = unsafe { (*gen).polarity };
                    let fresh = self.fresh_type(self.scope, polarity);
                    *self.generic_substitutions.get_or_insert(generic) = fresh;
                }
            }

            for &generic_pack in sub_fn.generic_packs.iter() {
                let generic_pack = unsafe { follow_type_pack_id(generic_pack) };
                let gen = unsafe { get_type_pack_id::<GenericTypePack>(generic_pack) };
                if !gen.is_null() {
                    let polarity = unsafe { (*gen).polarity };
                    let fresh = self.fresh_type_pack(self.scope, polarity);
                    *self.generic_pack_substitutions.get_or_insert(generic_pack) = fresh;
                }
            }
        }

        let arg_result = self.unify_type_pack_id_type_pack_id(super_fn.arg_types, sub_fn.arg_types);
        let ret_result = self.unify_type_pack_id_type_pack_id(sub_fn.ret_types, super_fn.ret_types);
        arg_result & ret_result
    }
}
