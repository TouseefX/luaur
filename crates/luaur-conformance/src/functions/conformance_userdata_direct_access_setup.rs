use crate::functions::conformance_userdata_direct_access_useratom::conformance_userdata_direct_access_useratom;
use crate::functions::lua_vec_2_push::kTagVec2;
use crate::functions::lua_vertex_push::kTagVertex;
use crate::functions::setup_userdata_helpers::setupUserdataHelpers;
use crate::functions::setup_vector_helpers::setup_vector_helpers;
use crate::functions::vec_2_direct_index::vec2DirectIndex;
use crate::functions::vec_2_direct_namecall::vec2DirectNamecall;
use crate::functions::vec_2_direct_newindex::vec2DirectNewindex;
use crate::functions::vertex_direct_index::vertex_direct_index;
use crate::functions::vertex_direct_namecall::vertex_direct_namecall;
use crate::functions::vertex_direct_newindex::vertexDirectNewindex;
use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::functions::lua_registeruserdatadirectaccess::lua_registeruserdatadirectaccess;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_userdata_direct_access_setup(l: *mut lua_State) {
    (*lua_callbacks(l)).useratom = Some(conformance_userdata_direct_access_useratom);

    setup_vector_helpers(l);
    setupUserdataHelpers(l);

    let vec2_ok = lua_registeruserdatadirectaccess(
        l,
        kTagVec2,
        Some(vec2DirectIndex),
        Some(vec2DirectNewindex),
        Some(vec2DirectNamecall),
    );
    assert_eq!(vec2_ok, 1);

    let vertex_ok = lua_registeruserdatadirectaccess(
        l,
        kTagVertex as i32,
        Some(vertex_direct_index),
        Some(vertexDirectNewindex),
        Some(vertex_direct_namecall),
    );
    assert_eq!(vertex_ok, 1);
}
