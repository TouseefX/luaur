use crate::functions::get_def::get_def_id;
use crate::records::blocked_type::BlockedType;
use crate::records::cell::Cell;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::phi::Phi;
use crate::records::scope::Scope;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn lookup(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        def: DefId,
        prototype: bool,
    ) -> Option<TypeId> {
        if unsafe { !get_def_id::<Cell>(def).is_null() } {
            return scope.lookup_def_id(def);
        }

        if let Some(phi) = unsafe { get_def_id::<Phi>(def).as_ref() } {
            if let Some(found) = scope.lookup_def_id(def) {
                return Some(found);
            } else if !prototype && phi.operands.len() == 1 {
                return self.lookup(scope, location, phi.operands[0], prototype);
            } else if !prototype {
                return None;
            }

            let mut res = unsafe { (*self.builtin_types).neverType };

            for operand in &phi.operands {
                let mut ty = self.lookup(scope, location, *operand, /*prototype*/ false);
                if ty.is_none() {
                    let blocked_ty = unsafe {
                        (*self.arena).add_type(BlockedType {
                            index: 0,
                            owner: core::ptr::null(),
                        })
                    };
                    self.local_types.try_insert(blocked_ty, TypeIds::type_ids());
                    unsafe {
                        *(*self.root_scope).lvalue_types.get_or_insert(*operand) = blocked_ty;
                    }
                    ty = Some(blocked_ty);
                }

                res = self.make_union_scope_ptr_location_type_id_type_id(
                    self.root_scope,
                    location,
                    res,
                    ty.unwrap(),
                );
            }

            unsafe {
                let scope_ptr = scope.as_ref() as *const Scope as *mut Scope;
                *(*scope_ptr).lvalue_types.get_or_insert(def) = res;
            }
            return Some(res);
        }

        unsafe { (*self.ice).ice_string("ConstraintGenerator::lookup is inexhaustive?") };
        None
    }
}
