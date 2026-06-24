use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::quantify::quantify;
use crate::records::function_type::FunctionType;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn quantify(&mut self, scope: &ScopePtr, ty: TypeId, _location: Location) -> TypeId {
        let ty = unsafe { follow_type_id(ty) };

        let ftv = unsafe { get_type_id::<FunctionType>(ty) };

        if !ftv.is_null() {
            quantify(ty, scope.level);
        }

        ty
    }
}
