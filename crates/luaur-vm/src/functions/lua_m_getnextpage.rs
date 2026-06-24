use crate::records::lua_page::lua_Page;

#[no_mangle]
pub unsafe fn luaM_getnextpage(page: *mut lua_Page) -> *mut lua_Page {
    (*page).listnext
}
