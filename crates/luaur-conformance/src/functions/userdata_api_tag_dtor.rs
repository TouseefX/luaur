use core::ffi::c_void;
use core::sync::atomic::Ordering;

use luaur_vm::records::lua_state::lua_State;

use crate::functions::userdata_api_dtor_hits::USERDATA_API_DTOR_HITS;

pub unsafe extern "C" fn userdata_api_tag_dtor(_l: *mut lua_State, data: *mut c_void) {
    USERDATA_API_DTOR_HITS.fetch_add(*(data as *const i32), Ordering::SeqCst);
}
