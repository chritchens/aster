use crate::result::Result;
use crate::syntax::Keyword;
use crate::typing::Type;
use crate::value::Value;
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct STElement {
    pub name: Option<String>,
    pub value: Value,
    pub file: Option<String>,
}

impl STElement {
    pub fn new() -> Self {
        STElement::default()
    }

    pub fn from_value(value: &Value) -> Self {
        STElement {
            name: value.name.clone(),
            value: value.clone(),
            file: value.token.file(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub files: BTreeSet<String>,
    pub incl_paths: BTreeSet<String>,
    pub def_types: BTreeSet<String>,
    pub def_values: BTreeSet<String>,

    pub includes: BTreeMap<String, Vec<STElement>>,
    pub types: BTreeMap<String, Vec<STElement>>,
    pub sigs: BTreeMap<String, Vec<STElement>>,
    pub prims: BTreeMap<String, Vec<STElement>>,
    pub sums: BTreeMap<String, Vec<STElement>>,
    pub prods: BTreeMap<String, Vec<STElement>>,
    pub funs: BTreeMap<String, Vec<STElement>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn from_values(values: &Values) -> Result<Self> {
        let mut st = SymbolTable::new();

        for value in values.clone().into_iter() {
            if let Some(file) = value.token.file() {
                st.files.insert(file);
            }

            if let Some(Type::App(types)) = value.typing.clone() {
                if types[0] == Type::Builtin {
                    let arg = value.children[1].name.clone().unwrap();
                    let keyword = Keyword::from_str(&value.clone().name.unwrap())?;

                    match keyword {
                        Keyword::Include => {
                            st.incl_paths.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.includes
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Deftype => {
                            st.def_types.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.types
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defsig => {
                            st.def_types.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.sigs
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defprim => {
                            st.def_values.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.prims
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defsum => {
                            st.def_values.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.sums
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defprod => {
                            st.def_values.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.prods
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defun => {
                            st.def_values.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.funs
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(st)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn symbol_table_from_values() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(include std.io)";

        let values = Values::from_str(s).unwrap();

        let res = SymbolTable::from_values(&values);

        assert!(res.is_ok());
    }

    #[test]
    fn symbol_table_includes() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(include std.io)";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.incl_paths.len(), 1);
        assert!(st.incl_paths.contains("std.io"));
        assert_eq!(st.includes.len(), 1);
        assert!(st.includes.contains_key("std.io"));
    }

    #[test]
    fn symbol_table_types() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(deftype RGB (Prod UInt UInt UInt))";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_types.len(), 1);
        assert!(st.def_types.contains("RGB"));
        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));
    }
}
