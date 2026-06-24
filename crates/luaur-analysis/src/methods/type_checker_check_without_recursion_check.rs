use crate::enums::solver_mode::SolverMode;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::freeze::freeze;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::synthesize_export_return::synthesize_export_return;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::module::Module;
use crate::records::source_module::SourceModule;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use luaur_ast::enums::mode::Mode;
use luaur_ast::records::location::Location;
use luaur_common::{FFlag, FInt};

// `unifierState.cachedUnifyError.clear()` (a `DenseHashMap<.., TypeErrorData>`)
// needs the value type to be default-constructible for empty slots, modelled in
// the Rust port by `DenseDefault`. The default sentinel is never read as a real
// error.
impl luaur_common::records::dense_hash_table::DenseDefault
    for crate::type_aliases::type_error_data::TypeErrorData
{
    fn dense_default() -> Self {
        crate::type_aliases::type_error_data::TypeErrorData::CodeTooComplex(
            crate::records::code_too_complex::CodeTooComplex::default(),
        )
    }
}

impl TypeChecker {
    pub fn check_without_recursion_check(
        &mut self,
        module: &SourceModule,
        mode: Mode,
        environment_scope: Option<ScopePtr>,
    ) -> ModulePtr {
        let mut new_module = Module::default();
        new_module.name = module.name.clone();
        new_module.human_readable_name = module.human_readable_name.clone();
        new_module.r#type = module.r#type;
        new_module.allocator = Some(module.allocator.clone());
        new_module.names = Some(module.names.clone());
        new_module.root = module.root;

        let current_module: ModulePtr = alloc::sync::Arc::new(new_module);
        self.current_module = Some(current_module.clone());

        // currentModule->internalTypes.owningModule = currentModule.get();
        // currentModule->interfaceTypes.owningModule = currentModule.get();
        unsafe {
            let module_mut =
                alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
            (*module_mut).internal_types.owning_module = module_mut;
            (*module_mut).interface_types.owning_module = module_mut;
        }

        unsafe {
            (*self.ice_handler).module_name = module.name.clone();
            self.normalizer.arena =
                &mut (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                    as *mut Module))
                    .internal_types as *mut crate::records::type_arena::TypeArena;
        }

        self.unifier_state.counters.recursion_limit =
            FInt::LuauTypeInferRecursionLimit.get() as i32;
        self.unifier_state.counters.iteration_limit = match self.unifier_iteration_limit {
            Some(limit) => limit,
            None => FInt::LuauTypeInferIterationLimit.get() as i32,
        };

        let parent_scope =
            environment_scope.unwrap_or_else(|| unsafe { (*self.global_scope).clone() });
        let module_scope: ScopePtr =
            alloc::sync::Arc::new(crate::records::scope::Scope::new(&parent_scope, 0));

        unsafe {
            let module_scope_mut =
                alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
            (*module_scope_mut).return_type = self.fresh_type_pack_scope_ptr(module_scope.clone());
            (*module_scope_mut).vararg_pack =
                Some(self.fresh_type_pack_scope_ptr(module_scope.clone()));
        }

        unsafe {
            let module_mut =
                alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
            (*module_mut)
                .scopes
                .push(((*module.root).base.base.location, module_scope.clone()));
            (*module_mut).mode = mode;
        }

        if let Some(prepare_module_scope) = &self.prepare_module_scope {
            let module_name = self.current_module.as_ref().unwrap().name.clone();
            prepare_module_scope(&module_name, &module_scope);
        }

        self.check_block(&module_scope, unsafe { &*module.root });

        if FFlag::LuauExportValueSyntax.get()
            && FFlag::LuauExportValueTypecheck.get()
            && !self.current_module.as_ref().unwrap().timeout
            && !self.current_module.as_ref().unwrap().cancelled
        {
            unsafe {
                let module_mut =
                    alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
                synthesize_export_return(self.builtin_types, module_mut);
            }
        }

        let module_return_type = module_scope.return_type;
        if !unsafe { get_type_pack_id::<FreeTypePack>(follow_type_pack_id(module_return_type)) }
            .is_null()
        {
            let empty_pack = self.add_type_pack_type_pack(TypePack {
                head: alloc::vec::Vec::new(),
                tail: None,
            });
            unsafe {
                let module_scope_mut =
                    alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
                (*module_scope_mut).return_type = empty_pack;
            }
        } else {
            let anyified = self.anyify_scope_ptr_type_pack_id_location(
                module_scope.clone(),
                module_return_type,
                Location::default(),
            );
            unsafe {
                let module_scope_mut =
                    alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
                (*module_scope_mut).return_type = anyified;
            }
        }

        let anyified_generics =
            self.anyify_module_return_type_pack_generics(module_scope.return_type);
        unsafe {
            let module_scope_mut =
                alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
            (*module_scope_mut).return_type = anyified_generics;
        }

        let keys: alloc::vec::Vec<crate::type_aliases::name_type_infer::Name> = module_scope
            .exported_type_bindings
            .keys()
            .cloned()
            .collect();
        for key in keys {
            let ty = module_scope
                .exported_type_bindings
                .get(&key)
                .unwrap()
                .r#type;
            let anyified = self.anyify_scope_ptr_type_id_location(
                module_scope.clone(),
                ty,
                Location::default(),
            );
            unsafe {
                let module_scope_mut =
                    alloc::sync::Arc::as_ptr(&module_scope) as *mut crate::records::scope::Scope;
                module_scope_mut
                    .as_mut()
                    .unwrap()
                    .exported_type_bindings
                    .get_mut(&key)
                    .unwrap()
                    .r#type = anyified;
            }
        }

        unsafe {
            let errors = &mut (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut Module))
                .errors as *mut crate::type_aliases::error_vec::ErrorVec;
            self.prepare_errors_for_display(&mut *errors);
        }

        // Clear the normalizer caches, since they contain types from the internal type surface
        self.normalizer.clear_caches();
        self.normalizer.arena = core::ptr::null_mut();

        unsafe {
            let module_mut =
                alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
            let ice = &mut *self.ice_handler;
            (*module_mut).clone_public_interface(self.builtin_types, ice, SolverMode::Old);

            freeze(&mut (*module_mut).internal_types);
            freeze(&mut (*module_mut).interface_types);
        }

        // Clear unifier cache since it's keyed off internal types that get deallocated
        self.unifier_state.cached_unify.clear();
        self.unifier_state.cached_unify_error.clear();
        self.unifier_state.skip_cache_for_type.clear();

        self.duplicate_type_aliases.clear();
        self.incorrect_extern_type_definitions.clear();

        let result = self.current_module.take().unwrap();
        result
    }
}
