//! Node: `cxx:Method:Luau.Analysis:Analysis/src/Subtyping.cpp:585:Subtyping`
//! Source: `Analysis/src/Subtyping.cpp:585-598` (hand-ported)

use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl Subtyping {
    /// C++ `Subtyping::Subtyping(NotNull<BuiltinTypes>, NotNull<TypeArena>,
    /// NotNull<Normalizer>, NotNull<TypeFunctionRuntime>,
    /// NotNull<InternalErrorReporter>)` — sets the five collaborators; every
    /// other member is default-constructed.
    pub fn subtyping_owned(
        builtin_types: *mut BuiltinTypes,
        type_arena: *mut TypeArena,
        normalizer: *mut Normalizer,
        type_function_runtime: *mut TypeFunctionRuntime,
        ice_reporter: *mut InternalErrorReporter,
    ) -> Self {
        Subtyping {
            builtin_types,
            arena: type_arena,
            normalizer,
            type_function_runtime,
            ice_reporter,
            limits: TypeCheckLimits::default(),
            unique_types: core::ptr::null(),
            seen_types: DenseHashMap::new((core::ptr::null(), core::ptr::null())),
            seen_packs: DenseHashMap::new((core::ptr::null(), core::ptr::null())),
            result_cache: DenseHashMap::new((core::ptr::null(), core::ptr::null())),
        }
    }
}
