use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::scope::Scope;
use crate::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl NonStrictTypeChecker {
    pub fn visit_ast_type_pack_generic(&mut self, tp: *mut AstTypePackGeneric) {
        let location = unsafe { (*tp).base.base.location };
        // C++ `Scope* scope = findInnermostScope(tp->location); LUAU_ASSERT(scope);`
        let scope_ptr = self.find_innermost_scope(location);
        LUAU_ASSERT!(!scope_ptr.is_null());
        let scope = unsafe { &*scope_ptr };

        let generic_name = unsafe { (*tp).generic_name };
        let name = unsafe {
            core::ffi::CStr::from_ptr(generic_name.value)
                .to_string_lossy()
                .into_owned()
        };

        if scope.lookup_pack(&name).is_some() {
            return;
        }

        if scope.lookup_type(&name).is_some() {
            let kind = SwappedGenericTypeParameter::Pack;
            let error = SwappedGenericTypeParameter {
                name: name.to_string(),
                kind,
            };
            let location_ref = &location;
            self.report_error(error.into(), location_ref);
            return;
        }

        let context = crate::records::unknown_symbol::Context::Type;
        let error = UnknownSymbol::new(name.to_string(), context);
        let location_ref = &location;
        self.report_error(error.into(), location_ref);
    }
}
