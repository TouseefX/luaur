use crate::functions::get_resource_path_0::get_resource_path_0;

pub fn get_resource_path() -> Option<alloc::string::String> {
    static mut PATH0: Option<alloc::string::String> = None;
    static INIT: std::sync::Once = std::sync::Once::new();

    unsafe {
        INIT.call_once(|| {
            PATH0 = get_resource_path_0();
        });
        PATH0.clone()
    }
}
