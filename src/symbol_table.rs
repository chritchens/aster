use crate::error::{Error, SemanticError};
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
    pub imp_paths: BTreeSet<String>,
    pub exp_defs: BTreeSet<String>,
    pub def_types: BTreeSet<String>,
    pub def_prims: BTreeSet<String>,
    pub def_sums: BTreeSet<String>,
    pub def_prods: BTreeSet<String>,
    pub def_sigs: BTreeSet<String>,
    pub def_funs: BTreeSet<String>,
    pub def_attrs: BTreeSet<String>,

    pub imports: BTreeMap<String, Vec<STElement>>,
    pub exports: BTreeMap<String, Vec<STElement>>,
    pub types: BTreeMap<String, Vec<STElement>>,
    pub prims: BTreeMap<String, Vec<STElement>>,
    pub sums: BTreeMap<String, Vec<STElement>>,
    pub prods: BTreeMap<String, Vec<STElement>>,
    pub sigs: BTreeMap<String, Vec<STElement>>,
    pub funs: BTreeMap<String, Vec<STElement>>,
    pub attrs: BTreeMap<String, Vec<STElement>>,

    pub main_sig: Option<STElement>,
    pub main_fun: Option<STElement>,
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
                    let keyword = Keyword::from_str(&value.clone().name.unwrap())?;

                    match keyword {
                        Keyword::Import => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.imp_paths.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.imports
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Export => {
                            let value = value.children[1].clone();

                            if value.children.len() > 1 {
                                let len = value.children.len();

                                for idx in 1..len {
                                    let child = value.children[idx].clone();

                                    let arg = child.name.clone().unwrap();
                                    st.exp_defs.insert(arg.clone());

                                    let st_el = STElement::from_value(&value);

                                    st.exports
                                        .entry(arg)
                                        .and_modify(|v| v.push(st_el.clone()))
                                        .or_insert_with(|| vec![st_el]);
                                }
                            } else {
                                let arg = value.name.clone().unwrap();
                                st.exp_defs.insert(arg.clone());

                                let st_el = STElement::from_value(&value);

                                st.exports
                                    .entry(arg)
                                    .and_modify(|v| v.push(st_el.clone()))
                                    .or_insert_with(|| vec![st_el]);
                            }
                        }
                        Keyword::Deftype => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_types.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.types
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defsig => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_sigs.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.sigs
                                .entry(arg.clone())
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el.clone()]);

                            if arg == "main" {
                                if st.main_sig.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main signature".into(),
                                    }));
                                }

                                st.main_sig = Some(st_el);
                            }
                        }
                        Keyword::Defprim => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_prims.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.prims
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defsum => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_sums.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.sums
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defprod => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_prods.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.prods
                                .entry(arg)
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el]);
                        }
                        Keyword::Defun => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_funs.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.funs
                                .entry(arg.clone())
                                .and_modify(|v| v.push(st_el.clone()))
                                .or_insert_with(|| vec![st_el.clone()]);

                            if arg == "main" {
                                if st.main_fun.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main function".into(),
                                    }));
                                }

                                st.main_fun = Some(st_el);
                            }
                        }
                        Keyword::Defattrs => {
                            let arg = value.children[1].name.clone().unwrap();
                            st.def_attrs.insert(arg.clone());

                            let st_el = STElement::from_value(&value);

                            st.attrs
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

        let s = "(import std.io)";

        let values = Values::from_str(s).unwrap();

        let res = SymbolTable::from_values(&values);

        assert!(res.is_ok());
    }

    #[test]
    fn symbol_table_imports() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(import std.io)";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.imp_paths.len(), 1);
        assert!(st.imp_paths.contains("std.io"));
        assert_eq!(st.imports.len(), 1);
        assert!(st.imports.contains_key("std.io"));
    }

    #[test]
    fn symbol_table_exports() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(export >>)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exp_defs.len(), 1);
        assert!(st.exp_defs.contains(">>"));
        assert_eq!(st.exports.len(), 1);
        assert!(st.exports.contains_key(">>"));

        s = "(export (prod a b c))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exp_defs.len(), 3);
        assert!(st.exp_defs.contains("a"));
        assert!(st.exp_defs.contains("b"));
        assert!(st.exp_defs.contains("c"));
        assert_eq!(st.exports.len(), 3);
        assert!(st.exports.contains_key("b"));
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

    #[test]
    fn symbol_table_attrs() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(defattrs sum (prod attr1 attr2 attr3))";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_attrs.len(), 1);
        assert!(st.def_attrs.contains("sum"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));
    }

    #[test]
    fn symbol_table_main() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(defsig main (Fun IO IO))\n(defun main io (id io))";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.is_some());
        assert_eq!(st.def_sigs.len(), 1);
        assert!(st.def_sigs.contains("main"));
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.def_funs.len(), 1);
        assert!(st.def_funs.contains("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));
    }
}
