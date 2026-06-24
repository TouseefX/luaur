use crate::records::feedback_vector_fixture::FeedbackVectorFixture;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::macros::clvalue::clvalue;
use luaur_vm::macros::lua_isfunction::lua_isfunction;
use luaur_vm::records::proto::Proto;

impl FeedbackVectorFixture {
    pub fn load(&mut self) -> *mut Proto {
        let bytecode = self.bcb.get_bytecode();
        let l = self.lua_state();

        unsafe {
            let res = luau_load(
                l,
                c"=FeedbackVectorTest".as_ptr(),
                bytecode.as_ptr() as *const core::ffi::c_char,
                bytecode.len(),
                0,
            );

            assert!(res == 0 && lua_isfunction!(l, -1));

            let top = clvalue!((*l).top.sub(1));
            let proto = (&(*top).inner.l).p;
            proto
        }
    }
}
