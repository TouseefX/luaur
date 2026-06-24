use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::substitution::Substitution;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Substitution {
    pub fn replace_children_type_pack_id(&mut self, tp: TypePackId) {
        unsafe {
            LUAU_ASSERT!(tp == (*self.base.log).follow_type_pack_id(tp));
        }

        if self.base.ignore_children_type_pack_id(tp) {
            return;
        }

        if unsafe { (*tp).owningArena != self.arena } {
            return;
        }

        if !unsafe { get_mutable_type_pack_id::<TypePack>(tp) }.is_null() {
            let tpp = unsafe { &mut *get_mutable_type_pack_id::<TypePack>(tp) };
            for tv in tpp.head.iter_mut() {
                *tv = self.replace_type_id(*tv);
            }
            if let Some(tail) = tpp.tail {
                tpp.tail = Some(self.replace_type_pack_id(tail));
            }
        } else if !unsafe { get_mutable_type_pack_id::<VariadicTypePack>(tp) }.is_null() {
            let vtp = unsafe { &mut *get_mutable_type_pack_id::<VariadicTypePack>(tp) };
            vtp.ty = self.replace_type_id(vtp.ty);
        } else if !unsafe { get_mutable_type_pack_id::<TypeFunctionInstanceTypePack>(tp) }.is_null()
        {
            let tfitp =
                unsafe { &mut *get_mutable_type_pack_id::<TypeFunctionInstanceTypePack>(tp) };
            for t in tfitp.typeArguments.iter_mut() {
                *t = self.replace_type_id(*t);
            }
            for t in tfitp.packArguments.iter_mut() {
                *t = self.replace_type_pack_id(*t);
            }
        }
    }
}
