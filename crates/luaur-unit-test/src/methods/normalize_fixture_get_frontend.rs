//! @interface-stub
use crate::functions::register_hidden_types::register_hidden_types;
use crate::records::normalize_fixture::NormalizeFixture;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::normalizer::Normalizer;
use luaur_analysis::records::scope::Scope;

impl NormalizeFixture {
    pub fn get_frontend(&mut self) -> &mut Frontend {
        if self.base.frontend.is_none() || self.normalizer.is_none() {
            let solver_mode = {
                let frontend = self.base.get_frontend();
                register_hidden_types(frontend);
                frontend.get_luau_solver_mode()
            };

            let builtin_types = self.base.builtin_types;
            assert!(
                !builtin_types.is_null(),
                "NormalizeFixture::get_frontend expected builtin types"
            );

            let any_type_pack = unsafe { (*builtin_types).anyTypePack };
            self.global_scope = Some(alloc::sync::Arc::new(Scope::scope_type_pack_id(
                any_type_pack,
            )));

            self.normalizer = Some(Normalizer::new(
                &mut self.arena as *mut _,
                builtin_types,
                &mut self.unifier_state as *mut _,
                solver_mode,
                false,
            ));
        }

        self.base.get_frontend()
    }
}
