use crate::functions::add_refinement::add_refinement;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_tag_type_alt_b::has_tag;
use crate::functions::is_boolean::is_boolean;
use crate::functions::is_buffer::is_buffer;
use crate::functions::is_integer::is_integer;
use crate::functions::is_nil::is_nil;
use crate::functions::is_number::is_number;
use crate::functions::is_overloaded_function::is_overloaded_function;
use crate::functions::is_string::is_string;
use crate::functions::is_table_intersection::is_table_intersection;
use crate::functions::is_thread::is_thread;
use crate::functions::is_undecidable::is_undecidable;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::is_a_predicate::IsAPredicate;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_checker::TypeChecker;
use crate::records::type_guard_predicate::TypeGuardPredicate;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::l_value::LValue;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_id_predicate::TypeIdPredicate;

/// `kTypeofRootTag` (Type.h:1257).
const K_TYPEOF_ROOT_TAG: &str = "typeofRoot";

fn is_table_like(ty: TypeId) -> bool {
    is_table_intersection(ty)
        || unsafe { !get_type_id::<TableType>(ty).is_null() }
        || unsafe { !get_type_id::<MetatableType>(ty).is_null() }
}

fn is_function_like(ty: TypeId) -> bool {
    is_overloaded_function(ty) || unsafe { !get_type_id::<FunctionType>(ty).is_null() }
}

fn is_userdata_like(ty: TypeId) -> bool {
    unsafe { !get_type_id::<ExternType>(ty).is_null() }
}

impl TypeChecker {
    /// C++ helper lambda `refine` inside `resolve(const TypeGuardPredicate&, ...)`.
    fn type_guard_refine(
        &mut self,
        lvalue: &LValue,
        refis: &mut RefinementMap,
        scope: ScopePtr,
        sense: bool,
        f: fn(TypeId) -> bool,
        maps_to: Option<TypeId>,
    ) {
        let predicate: TypeIdPredicate = alloc::boxed::Box::new(move |ty: TypeId| -> Option<TypeId> {
            if sense && unsafe { !get_type_id::<UnknownType>(ty).is_null() } {
                return maps_to.or(Some(ty));
            }

            if f(ty) == sense {
                return Some(ty);
            }

            if is_undecidable(ty) {
                return maps_to.or(Some(ty));
            }

            None
        });

        self.refine_l_value(lvalue, refis, scope, predicate);
    }

    pub fn resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
        &mut self,
        typeguard_p: &TypeGuardPredicate,
        refis: &mut RefinementMap,
        scope: ScopePtr,
        sense: bool,
    ) {
        // Rewrite the predicate 'type(foo) == "vector"' to be 'typeof(foo) == "Vector3"'.
        // They're exactly identical.
        if !typeguard_p.is_typeof && typeguard_p.kind == "vector" {
            return self.resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
                &TypeGuardPredicate {
                    lvalue: typeguard_p.lvalue.clone(),
                    location: typeguard_p.location,
                    kind: "Vector3".to_string(),
                    is_typeof: true,
                },
                refis,
                scope,
                sense,
            );
        }

        let ty = self.resolve_l_value_refinement_map_scope_ptr_l_value(
            refis,
            scope.clone(),
            &typeguard_p.lvalue,
        );
        if ty.is_none() {
            return;
        }

        // In certain cases, the value may actually be nil, but Luau doesn't know about it.
        // So we whitelist this.
        if sense && typeguard_p.kind == "nil" {
            add_refinement(refis, &typeguard_p.lvalue, self.nil_type);
            return;
        }

        // Note: "vector" never happens here at this point.
        let kind = typeguard_p.kind.as_str();
        if kind == "nil" {
            // This can still happen when sense is false!
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_nil,
                Some(self.nil_type),
            );
        } else if kind == "string" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_string,
                Some(self.string_type),
            );
        } else if kind == "number" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_number,
                Some(self.number_type),
            );
        } else if kind == "integer" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_integer,
                Some(self.integer_type),
            );
        } else if kind == "boolean" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_boolean,
                Some(self.boolean_type),
            );
        } else if kind == "thread" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_thread,
                Some(self.thread_type),
            );
        } else if kind == "buffer" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_buffer,
                Some(self.buffer_type),
            );
        } else if kind == "table" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_table_like,
                None,
            );
        } else if kind == "function" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_function_like,
                None,
            );
        } else if kind == "userdata" {
            return self.type_guard_refine(
                &typeguard_p.lvalue,
                refis,
                scope,
                sense,
                is_userdata_like,
                None,
            );
        }

        if !typeguard_p.is_typeof {
            let err = self.error_recovery_type_scope_ptr(&scope);
            add_refinement(refis, &typeguard_p.lvalue, err);
            return;
        }

        let global_scope = unsafe { &**self.global_scope };
        let type_fun = global_scope.lookup_type(&typeguard_p.kind);
        let type_fun = match type_fun {
            Some(tf) if tf.type_params().is_empty() && tf.type_pack_params().is_empty() => tf,
            _ => {
                let err = self.error_recovery_type_scope_ptr(&scope);
                add_refinement(refis, &typeguard_p.lvalue, err);
                return;
            }
        };

        let resolved = unsafe { follow_type_id(type_fun.r#type()) };
        let extern_type_builtin = unsafe { (*self.builtin_types).externType };

        // You cannot refine to the top class type.
        if resolved == extern_type_builtin {
            let err = self.error_recovery_type_scope_ptr(&scope);
            add_refinement(refis, &typeguard_p.lvalue, err);
            return;
        }

        // We're only interested in the root type of any extern type.
        let etv = unsafe { get_type_id::<ExternType>(resolved) };
        let is_root_extern_type = !etv.is_null()
            && (unsafe { (*etv).parent } == Some(extern_type_builtin)
                || has_tag(resolved, K_TYPEOF_ROOT_TAG));
        if !is_root_extern_type {
            let err = self.error_recovery_type_scope_ptr(&scope);
            add_refinement(refis, &typeguard_p.lvalue, err);
            return;
        }

        // Until type filtering functions are broken out, we rewrite this to be the same as using IsA.
        self.resolve_is_a_predicate_refinement_map_scope_ptr_bool(
            &IsAPredicate {
                lvalue: typeguard_p.lvalue.clone(),
                location: typeguard_p.location,
                ty: resolved,
            },
            refis,
            scope,
            sense,
        );
    }
}
