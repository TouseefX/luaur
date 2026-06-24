use crate::enums::path_type::PathType;
use crate::records::repl_with_path_fixture::ReplWithPathFixture;
use alloc::string::String;
use luaur_cli_lib::functions::get_current_working_directory::get_current_working_directory;
use luaur_cli_lib::functions::get_parent_path::get_parent_path;
use luaur_cli_lib::functions::is_directory::is_directory;

use crate::functions::get_resource_path::get_resource_path;

pub fn repl_with_path_fixture_get_luau_directory(
    fixture: &ReplWithPathFixture,
    type_: PathType,
) -> String {
    let _ = fixture;

    // Require fixtures vendored alongside the crate (standalone / published-repo
    // layout). Checked before the cwd-walk that finds the upstream
    // luau/tests/require checkout in the original workspace. The require resolver
    // demands a `./`, `../` or `@` prefix, so return the vendored dir as a path
    // relative to CWD (CARGO_MANIFEST_DIR is at or under CWD for cargo/nextest,
    // so a "./..." form always exists).
    {
        let vendored_abs = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");
        if is_directory(&format!("{}/tests/require", vendored_abs)) {
            match type_ {
                // Cache keys are built from the absolute form; keep it absolute.
                PathType::Absolute => return String::from(vendored_abs),
                // Require calls need a ./ prefix; the relative form resolves back
                // to vendored_abs against CWD, so the cache key still matches.
                PathType::Relative => {
                    if let Some(cwd) = get_current_working_directory() {
                        let cwd = cwd.replace('\\', "/");
                        if vendored_abs == cwd {
                            return String::from(".");
                        }
                        if let Some(rel) = vendored_abs.strip_prefix(&format!("{}/", cwd)) {
                            return format!("./{}", rel);
                        }
                    }
                    return String::from(vendored_abs);
                }
            }
        }
    }

    let mut luau_dir_rel = String::from(".");
    let mut luau_dir_abs = String::new();

    #[cfg(target_os = "ios")]
    {
        let cwd0 = get_current_working_directory();
        let cwd = get_resource_path();
        if let (Some(res), Some(cwd_val)) = (cwd, cwd0) {
            if res.starts_with(&cwd_val) {
                luau_dir_rel = format!("./{}", &res[cwd_val.len()..]);
            }
        }
    }

    let cwd = get_current_working_directory();
    let cwd = cwd.expect("Error getting Luau path");
    let cwd_normalized = cwd.replace('\\', "/");
    luau_dir_abs = cwd_normalized;

    for _ in 0..20 {
        let engine_test_dir = is_directory(&format!("{}/Client/Luau/tests", luau_dir_abs));
        let luau_test_dir = is_directory(&format!("{}/tests/require", luau_dir_abs));
        // In the luau-rs workspace layout, the upstream fixtures live under a
        // nested `luau/` checkout rather than directly at `<dir>/tests/require`.
        let luau_subdir_test = is_directory(&format!("{}/luau/tests/require", luau_dir_abs));

        if engine_test_dir || luau_test_dir || luau_subdir_test {
            if engine_test_dir {
                luau_dir_rel = format!("{}/Client/Luau", luau_dir_rel);
                luau_dir_abs = format!("{}/Client/Luau", luau_dir_abs);
            } else if luau_subdir_test {
                luau_dir_rel = format!("{}/luau", luau_dir_rel);
                luau_dir_abs = format!("{}/luau", luau_dir_abs);
            }

            match type_ {
                PathType::Relative => return luau_dir_rel,
                PathType::Absolute => return luau_dir_abs,
            }
        }

        if luau_dir_rel == "." {
            luau_dir_rel = "..".to_string();
        } else {
            luau_dir_rel = format!("{}/..", luau_dir_rel);
        }

        let parent_path = get_parent_path(&luau_dir_abs);
        let parent_path = parent_path.expect("Error getting Luau path");
        luau_dir_abs = parent_path;
    }

    panic!("Error getting Luau path");
}
