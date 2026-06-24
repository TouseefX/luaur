#[cfg(not(target_os = "windows"))]
#[allow(non_upper_case_globals)]
pub const REGISTER_FRAME_WEAK: &str = "weak";

#[cfg(target_os = "windows")]
#[allow(non_upper_case_globals)]
pub const REGISTER_FRAME_WEAK: () = ();
