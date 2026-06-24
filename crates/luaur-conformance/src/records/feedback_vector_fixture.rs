use crate::functions::alloc::alloc as luau_alloc;
use crate::type_aliases::state_ref::StateRef;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_vm::functions::lua_newstate::lua_newstate;
use luaur_vm::records::closure::Closure;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::records::proto::Proto;

#[allow(non_camel_case_types)]
pub struct FeedbackVectorFixture {
    pub bcb: BytecodeBuilder,
    pub l: StateRef,
    pub on_inline:
        Option<unsafe extern "C" fn(*mut lua_State, *mut Closure, *mut Closure, u32) -> *mut Proto>,
}

impl FeedbackVectorFixture {
    pub fn new() -> Self {
        let state = unsafe { lua_newstate(Some(luau_alloc), core::ptr::null_mut()) };
        let l = StateRef::new(state).expect("lua_newstate failed");

        Self {
            bcb: BytecodeBuilder::new(None),
            l,
            on_inline: None,
        }
    }

    pub fn lua_state(&self) -> *mut lua_State {
        self.l.as_ptr()
    }
}
