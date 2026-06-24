use crate::records::builtin_types::BuiltinTypes;
use crate::records::function_type::FunctionType;
use crate::records::normalized_type::NormalizedType;
use crate::records::normalizer::Normalizer;
use crate::records::type_arena::TypeArena;
use crate::records::type_function_inference_result::TypeFunctionInferenceResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guess_result::TypeFunctionReductionGuessResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

#[derive(Debug, Clone)]
pub struct TypeFunctionReductionGuesser {
    pub(crate) function_reduces_to: DenseHashMap<TypeId, TypeId>,
    pub(crate) substitutable: DenseHashMap<TypeId, TypeId>,
    pub(crate) to_infer: VecDeque<TypeId>,
    pub(crate) cyclic_instances: DenseHashSet<TypeId>,
    pub(crate) arena: *mut TypeArena,
    pub(crate) builtins: *mut BuiltinTypes,
    pub(crate) normalizer: *mut Normalizer,
}
