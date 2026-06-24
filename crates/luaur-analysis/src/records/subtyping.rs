use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::seen_set_subtyping::SeenSet;
use crate::type_aliases::seen_type_pack_set::SeenTypePackSet;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct Subtyping {
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) arena: *mut TypeArena,
    pub(crate) normalizer: *mut Normalizer,
    pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
    pub(crate) ice_reporter: *mut InternalErrorReporter,
    pub(crate) limits: TypeCheckLimits,
    pub(crate) unique_types: *const DenseHashSet<TypeId>,
    pub(crate) seen_types: SeenSet,
    pub(crate) seen_packs: SeenTypePackSet,
    pub(crate) result_cache: DenseHashMap<(TypeId, TypeId), SubtypingResult, TypePairHash>,
}
