use crate::records::builtin_types::BuiltinTypes;
use crate::records::normalizer::Normalizer;
use crate::records::type_arena::TypeArena;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

impl TypeFunctionReductionGuesser {
    pub fn type_function_reduction_guesser_type_function_reduction_guesser(
        arena: *mut TypeArena,
        builtins: *mut BuiltinTypes,
        normalizer: *mut Normalizer,
    ) -> Self {
        Self {
            function_reduces_to: DenseHashMap::new(core::ptr::null_mut()),
            substitutable: DenseHashMap::new(core::ptr::null_mut()),
            to_infer: VecDeque::new(),
            cyclic_instances: DenseHashSet::new(core::ptr::null_mut()),
            arena,
            builtins,
            normalizer,
        }
    }
}
