use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn slice_type_pack(
    slice_index: usize,
    to_be_sliced: TypePackId,
    head: &[TypeId],
    tail: Option<TypePackId>,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
) -> TypePackId {
    if slice_index == 0 {
        to_be_sliced
    } else if slice_index == head.len() {
        tail.unwrap_or(builtin_types.emptyTypePack)
    } else {
        let mut head_slice = alloc::vec::Vec::new();
        let mut iter = begin(to_be_sliced);
        let end_iter = end_type_pack_id(to_be_sliced);

        for _ in 0..slice_index {
            if iter.operator_ne(&end_iter) {
                iter.operator_inc();
            }
        }

        while iter.operator_ne(&end_iter) {
            head_slice.push(*iter.operator_deref());
            iter.operator_inc();
        }

        arena.add_type_pack_vector_type_id_optional_type_pack_id(head_slice, tail)
    }
}
