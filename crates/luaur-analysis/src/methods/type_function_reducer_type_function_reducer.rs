use crate::records::function_graph_reduction_result::FunctionGraphReductionResult;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_type_pack_id_set::TypeOrTypePackIdSet;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;
use luaur_common::records::vec_deque::VecDeque;

impl TypeFunctionReducer {
    pub fn type_function_reducer(
        queued_tys: VecDeque<TypeId>,
        queued_tps: VecDeque<TypePackId>,
        should_guess: TypeOrTypePackIdSet,
        cyclic_types: Vec<TypeId>,
        location: Location,
        ctx: NonNull<TypeFunctionContext>,
        force: bool,
    ) -> Self {
        Self {
            ctx,
            queued_tys,
            queued_tps,
            should_guess,
            cyclic_type_functions: cyclic_types,
            irreducible: TypeOrTypePackIdSet::new(core::ptr::null_mut()),
            result: FunctionGraphReductionResult {
                errors: alloc::vec::Vec::new(),
                messages: alloc::vec::Vec::new(),
                blocked_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null(),
                ),
                blocked_packs: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null(),
                ),
                reduced_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null(),
                ),
                reduced_packs: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null(),
                ),
                irreducible_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null(),
                ),
            },
            force,
            location,
        }
    }
}
