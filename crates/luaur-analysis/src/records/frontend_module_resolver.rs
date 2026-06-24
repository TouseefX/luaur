//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Frontend.h:133:frontend_module_resolver`
//! Source: `Analysis/include/Luau/Frontend.h`
//!
//! C++ `struct FrontendModuleResolver : ModuleResolver` (Frontend.h:133-151).
//! The base `ModuleResolver` interface is pure-virtual; its overrides live as
//! `FrontendModuleResolver` methods. The data members are ported here.

use crate::records::frontend::Frontend;
use crate::records::module_resolver::ModuleResolver;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::module_ptr_module_resolver::ModulePtr;
use std::collections::HashMap;
use std::sync::Mutex;

#[repr(C)]
pub struct FrontendModuleResolver {
    pub base: ModuleResolver,

    /// `Frontend* frontend;`
    pub frontend: *mut Frontend,

    /// `mutable std::mutex moduleMutex;`
    pub module_mutex: Mutex<()>,

    /// `std::unordered_map<ModuleName, ModulePtr> modules;`
    pub modules: HashMap<ModuleName, ModulePtr>,
}

impl core::fmt::Debug for FrontendModuleResolver {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FrontendModuleResolver")
            .field("modules", &self.modules)
            .finish_non_exhaustive()
    }
}
