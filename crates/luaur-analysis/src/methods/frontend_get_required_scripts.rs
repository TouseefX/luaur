use crate::functions::trace_requires::trace_requires;
use crate::records::file_resolver::FileResolver;
use crate::records::frontend::Frontend;
use crate::records::require_trace_result::RequireTraceResult;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::vec::Vec;
use luaur_config::records::config::Config;

impl Frontend {
    pub fn get_required_scripts(
        &mut self,
        name: &ModuleName,
        limits: &TypeCheckLimits,
    ) -> Vec<ModuleName> {
        // C++: RequireTraceResult require = requireTrace[name];
        // operator[] default-inserts an empty value when the key is absent.
        let mut require = self
            .require_trace
            .entry(name.clone())
            .or_insert_with(|| RequireTraceResult {
                exprs: luaur_common::records::dense_hash_map::DenseHashMap::new(
                    core::ptr::null_mut(),
                ),
                require_list: Vec::new(),
            })
            .clone();

        if self.is_dirty(name, false) {
            let source = unsafe { FileResolver::read_source(self.file_resolver, name) };
            if source.is_none() {
                return Vec::new();
            }

            let source_code = source.unwrap();
            let config: &Config = unsafe {
                let get_config = (*self.config_resolver)
                    .get_config
                    .expect("ConfigResolver::getConfig is not set");
                &*get_config(self.config_resolver, name, limits)
            };
            let mut opts = config.parse_options.clone();
            opts.capture_comments = true;
            let mut result =
                self.parse_module_name_string_view_parse_options(name, &source_code.source, &opts);
            result.r#type = source_code.r#type;
            require = trace_requires(self.file_resolver, result.root, name.clone(), limits);
        }

        let mut required_module_names: Vec<ModuleName> = Vec::new();
        required_module_names.reserve(require.require_list.len());
        for (module_name, _) in require.require_list.iter() {
            required_module_names.push(module_name.clone());
        }
        required_module_names
    }
}
