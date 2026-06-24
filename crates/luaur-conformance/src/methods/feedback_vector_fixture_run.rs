use crate::records::feedback_vector_fixture::FeedbackVectorFixture;
use luaur_vm::functions::lua_resume::lua_resume;

impl FeedbackVectorFixture {
    pub fn run(&mut self) {
        let l = self.lua_state();

        unsafe {
            (*(*l).global).ecb.inlinefunction = self.on_inline;

            let status = lua_resume(l, core::ptr::null_mut(), 0);
            assert_eq!(status, 0);
        }
    }
}
