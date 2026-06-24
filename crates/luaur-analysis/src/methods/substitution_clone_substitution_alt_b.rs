use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::substitution::Substitution;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl Substitution {
    pub fn clone_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
        let mut tp = unsafe { (*self.base.log).follow_type_pack_id(tp) };

        let ptp = unsafe { (*self.base.log).pending_type_pack_id(tp) };
        if !ptp.is_null() {
            tp = unsafe { &(*ptp).pending as *const TypePackVar };
        }

        let tpp = unsafe { get_type_pack_id::<TypePack>(tp) };
        if !tpp.is_null() {
            let tpp = unsafe { &*tpp };
            let clone = TypePack {
                head: tpp.head.clone(),
                tail: tpp.tail,
            };
            return self.add_type_pack(clone);
        }

        let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(tp) };
        if !vtp.is_null() {
            let vtp = unsafe { &*vtp };
            let clone = VariadicTypePack {
                ty: vtp.ty,
                hidden: vtp.hidden,
            };
            return self.add_type_pack(clone);
        }

        let tfitp = unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) };
        if !tfitp.is_null() {
            let tfitp = unsafe { &*tfitp };
            let clone = TypeFunctionInstanceTypePack {
                function: tfitp.function,
                typeArguments: tfitp.typeArguments.clone(),
                packArguments: tfitp.packArguments.clone(),
            };
            return self.add_type_pack(clone);
        }

        self.add_type_pack(unsafe { (*tp).clone() })
    }
}
