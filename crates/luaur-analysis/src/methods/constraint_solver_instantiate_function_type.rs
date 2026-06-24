//! `TypeId ConstraintSolver::instantiateFunctionType(TypeId functionTypeId,
//!   const std::vector<TypeId>& typeArguments, const std::vector<TypePackId>& typePackArguments,
//!   NotNull<Scope> scope, const Location& location)`
//! (`Analysis/src/ConstraintSolver.cpp:3193-3267`, hand-ported faithfully).

use alloc::vec::Vec;

use crate::enums::polarity::Polarity;
use crate::functions::follow_type::follow_type_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::records::clone_state::CloneState;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::function_type::FunctionType;
use crate::records::replacer::Replacer;
use crate::records::scope::Scope;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ConstraintSolver {
    pub fn instantiate_function_type(
        &mut self,
        function_type_id: TypeId,
        type_arguments: &Vec<TypeId>,
        type_pack_arguments: &Vec<TypePackId>,
        scope: *mut Scope,
        _location: &Location,
    ) -> TypeId {
        let function_type_id = unsafe { follow_type_id(function_type_id) };

        // no work to be done if we're not instantiating with anything
        if type_arguments.is_empty() && type_pack_arguments.is_empty() {
            return function_type_id;
        }

        let ft = unsafe { get_type_id::<FunctionType>(function_type_id) };
        if ft.is_null() {
            return function_type_id;
        }

        let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::new(core::ptr::null());
        let generics = unsafe { &(*ft).generics };
        let mut type_parameters_iter = 0usize;

        for &type_argument in type_arguments.iter() {
            if type_parameters_iter == generics.len() {
                break;
            }

            *replacements.get_or_insert(generics[type_parameters_iter]) = type_argument;
            type_parameters_iter += 1;
        }

        while type_parameters_iter != generics.len() {
            let fresh = fresh_type(
                unsafe { &mut *self.arena },
                unsafe { &*self.builtin_types },
                scope,
                Polarity::Mixed,
            );
            *replacements.get_or_insert(generics[type_parameters_iter]) = fresh;
            type_parameters_iter += 1;
        }

        let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> =
            DenseHashMap::new(core::ptr::null());
        let generic_packs = unsafe { &(*ft).generic_packs };
        let mut type_pack_parameters_iter = 0usize;

        for &type_pack_argument in type_pack_arguments.iter() {
            if type_pack_parameters_iter == generic_packs.len() {
                break;
            }

            *replacement_packs.get_or_insert(generic_packs[type_pack_parameters_iter]) =
                type_pack_argument;
            type_pack_parameters_iter += 1;
        }

        let mut r = Replacer::replacer(
            self.arena,
            &mut replacements as *mut DenseHashMap<TypeId, TypeId>,
            &mut replacement_packs as *mut DenseHashMap<TypePackId, TypePackId>,
        );

        let mut cs = CloneState {
            builtin_types: self.builtin_types,
            seen_types: DenseHashMap::new(core::ptr::null()),
            seen_type_packs: DenseHashMap::new(core::ptr::null()),
        };

        // We clone persistent types here to enable instantiation for generic
        // builtins like `table.find`; otherwise, the lines after would
        // immediately corrupt the definitions of the original function.
        let cloned_function_type_id = shallow_clone(
            function_type_id,
            unsafe { &mut *self.arena },
            &mut cs,
            /* clonePersistentTypes */ true,
        );
        let ft2 = unsafe { get_mutable_type_id::<FunctionType>(cloned_function_type_id) };
        LUAU_ASSERT!(ft as *const FunctionType != ft2 as *const FunctionType);

        // We instantiate all generics, replacing any with free types.
        unsafe {
            (*ft2).generics.clear();

            // However, we only instantiate as many type pack arguments as are given.
            if !(*ft2).generic_packs.is_empty()
                && type_pack_arguments.len() < (*ft2).generic_packs.len()
            {
                (*ft2).generic_packs.drain(0..type_pack_arguments.len());
            } else {
                (*ft2).generic_packs.clear();
            }
        }

        match r.substitute_type_id(cloned_function_type_id) {
            Some(result) => result,
            None => unsafe { (*self.builtin_types).errorType },
        }
    }
}
