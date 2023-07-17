use crate::package_tree::Package;
use ahash::{AHashMap, AHashSet};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseState {
    Pending,
    ParseError,
    Warning,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompileState {
    Pending,
    Error,
    Warning,
    Success,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    pub path: String,
    pub parse_state: ParseState,
    pub compile_state: CompileState,
    pub last_modified: SystemTime,
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Implementation {
    pub path: String,
    pub parse_state: ParseState,
    pub compile_state: CompileState,
    pub last_modified: SystemTime,
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile {
    pub implementation: Implementation,
    pub interface: Option<Interface>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MlMap {
    pub dirty: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    SourceFile(SourceFile),
    MlMap(MlMap),
}

#[derive(Debug, Clone)]
pub struct Module {
    pub source_type: SourceType,
    pub deps: AHashSet<String>,
    pub reverse_deps: AHashSet<String>,
    pub package_name: String,
    pub compile_dirty: bool,
}

#[derive(Debug)]
pub struct BuildState {
    pub modules: AHashMap<String, Module>,
    pub packages: AHashMap<String, Package>,
    pub module_names: AHashSet<String>,
    pub project_root: String,
    pub root_config_name: String,
}

impl BuildState {
    pub fn get_package(&self, package_name: &str) -> Option<&Package> {
        self.packages.get(package_name)
    }

    pub fn get_module(&self, module_name: &str) -> Option<&Module> {
        self.modules.get(module_name)
    }
    pub fn new(
        project_root: String,
        root_config_name: String,
        packages: AHashMap<String, Package>,
    ) -> Self {
        Self {
            module_names: AHashSet::new(),
            modules: AHashMap::new(),
            packages: packages,
            project_root: project_root,
            root_config_name: root_config_name,
        }
    }
    pub fn insert_module(&mut self, module_name: &str, module: Module) {
        self.modules.insert(module_name.to_owned(), module);
        self.module_names.insert(module_name.to_owned());
    }
}
