pub fn const2<'a>(enabled: &'a str, disabled: &'a str) -> &'a str {
    if luaur_common::FFlag::LuauConst2.get() {
        enabled
    } else {
        disabled
    }
}
