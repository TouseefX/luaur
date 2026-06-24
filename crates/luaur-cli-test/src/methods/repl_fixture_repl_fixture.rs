use crate::functions::run_code::run_code;
use crate::functions::setup_state::setup_state;
use crate::records::repl_fixture::ReplFixture;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;

impl ReplFixture {
    pub fn new() -> Self {
        let l_state = lua_l_newstate();
        let l = l_state as *mut core::ffi::c_void;

        setup_state(l_state);
        unsafe {
            lua_l_sandboxthread(l_state);
        }

        let pretty_print_source = alloc::string::String::from(
            r#"
capturedoutput = ""

function arraytostring(arr)
    local strings = {}
    table.foreachi(arr, function(k,v) table.insert(strings, pptostring(v)) end )
    return "{" .. table.concat(strings, ", ") .. "}"
end

function pptostring(x)
    if type(x) == "table" then
        return arraytostring(x)
    elseif type(x) == "string" then
        return '"' .. x .. '"'
    else
        return tostring(x)
    end
end

function _PRETTYPRINT(...)
    local args = table.pack(...)
    local strings = {}
    for i=1, args.n do
        local item = args[i]
        local str = pptostring(item, customoptions)
        if i == 1 then
            capturedoutput = capturedoutput .. str
        else
            capturedoutput = capturedoutput .. "\t" .. str
        end
    end
end
"#,
        );
        run_code(l_state, &pretty_print_source);

        Self {
            lua_state: l,
            l,
            pretty_print_source,
        }
    }
}
