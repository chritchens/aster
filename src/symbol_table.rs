use crate::value::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct STElement {
    pub name: String,
    pub value: Value,
    pub file: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub files: BTreeSet<String>,
    pub value_names: BTreeSet<String>,
    pub type_names: BTreeSet<String>,
    pub path_names: BTreeSet<String>,

    pub includes: BTreeMap<String, Vec<STElement>>,
    pub types: BTreeMap<String, Vec<STElement>>,
    pub signature: BTreeMap<String, Vec<STElement>>,
    pub sums: BTreeMap<String, Vec<STElement>>,
    pub prods: BTreeMap<String, Vec<STElement>>,
    pub funs: BTreeMap<String, Vec<STElement>>,
}
