#[allow(non_camel_case_types)]
pub type luarequire_Configuration_init = Option<
    unsafe extern "C" fn(
        config: *mut crate::records::luarequire_configuration::luarequire_Configuration,
    ),
>;

pub type LuarequireConfigurationInit = luarequire_Configuration_init;
