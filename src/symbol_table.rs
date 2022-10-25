use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

pub struct SymbolTable {
    pub file: String,
    pub values: Values,

    // module -> set(value_idx)
    pub imported_modules: BTreeMap<String, BTreeSet<usize>>,

    // name -> module
    pub imported_values: BTreeMap<String, String>,
    pub imported_types: BTreeMap<String, String>,
    // name -> value_idx
    pub exported_values: BTreeMap<String, usize>,
    pub exported_types: BTreeMap<String, usize>,

    // name -> vec(value_idx) // path
    pub attrs_defs: BTreeMap<String, Vec<usize>>,
    // name -> set(attr)
    pub attrs: BTreeMap<String, BTreeSet<String>>,

    // name -> vec(value_idx) // path
    pub type_defs: BTreeMap<String, Vec<usize>>,
    pub sig_defs: BTreeMap<String, Vec<usize>>,
    pub prim_defs: BTreeMap<String, Vec<usize>>,
    pub sum_defs: BTreeMap<String, Vec<usize>>,
    pub prod_defs: BTreeMap<String, Vec<usize>>,
    pub fun_defs: BTreeMap<String, Vec<usize>>,
    pub app_defs: BTreeMap<String, Vec<usize>>,

    pub fun_apps: BTreeMap<String, Vec<usize>>,
    pub type_apps: BTreeMap<String, Vec<usize>>,

    pub let_scopes: BTreeMap<String, Vec<usize>>,
}

pub struct GlobalSymbolTable {
    pub files: BTreeSet<String>,

    // file -> symbol_table
    pub tables: BTreeMap<String, SymbolTable>,
}
