use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_utils::follow_optional_ty;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::records::type_pack::TypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl TypeFunctionReductionGuesser {
    pub fn guess_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
        let (head, tail) = flatten_type_pack_id(tp);

        let mut guessed_head: Vec<TypeId> = Vec::with_capacity(head.len());

        for typ in head.iter().copied() {
            let guessed_type: Option<TypeId> = self.guess_type(typ);

            if guessed_type.is_none() {
                return None;
            }

            let guess: TypeId = unsafe { follow_type_id(guessed_type.unwrap()) };
            let instance_ptr: *const TypeFunctionInstanceType =
                unsafe { get_type_id::<TypeFunctionInstanceType>(guess) };
            if !instance_ptr.is_null() {
                return None;
            }

            guessed_head.push(guessed_type.unwrap());
        }

        let pack = TypePack {
            head: guessed_head,
            tail,
        };

        Some(unsafe { (*self.arena).add_type_pack_t(pack) })
    }
}
