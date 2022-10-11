use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::syntax::Keyword;
use crate::typing::Type;
use crate::value::Value;
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

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
    pub def_apps: BTreeSet<String>,
    pub def_attrs: BTreeSet<String>,

    pub imports: BTreeMap<String, Vec<Value>>,
    pub exports: BTreeMap<String, Vec<Value>>,
    pub types: BTreeMap<String, Vec<Value>>,
    pub prims: BTreeMap<String, Vec<Value>>,
    pub sums: BTreeMap<String, Vec<Value>>,
    pub prods: BTreeMap<String, Vec<Value>>,
    pub sigs: BTreeMap<String, Vec<Value>>,
    pub funs: BTreeMap<String, Vec<Value>>,
    pub apps: BTreeMap<String, Vec<Value>>,
    pub attrs: BTreeMap<String, Vec<Value>>,

    pub main_type: Option<Value>,
    pub main_sig: Option<Value>,
    pub main_fun: Option<Value>,
    pub main_app: Option<Value>,
    pub main_attrs: Option<Value>,

    pub scoped_def_types: BTreeSet<String>,
    pub scoped_def_prims: BTreeSet<String>,
    pub scoped_def_sums: BTreeSet<String>,
    pub scoped_def_prods: BTreeSet<String>,
    pub scoped_def_sigs: BTreeSet<String>,
    pub scoped_def_funs: BTreeSet<String>,
    pub scoped_def_apps: BTreeSet<String>,
    pub scoped_def_attrs: BTreeSet<String>,

    pub scoped_types: BTreeMap<String, Vec<Value>>,
    pub scoped_prims: BTreeMap<String, Vec<Value>>,
    pub scoped_sums: BTreeMap<String, Vec<Value>>,
    pub scoped_prods: BTreeMap<String, Vec<Value>>,
    pub scoped_sigs: BTreeMap<String, Vec<Value>>,
    pub scoped_funs: BTreeMap<String, Vec<Value>>,
    pub scoped_apps: BTreeMap<String, Vec<Value>>,
    pub scoped_attrs: BTreeMap<String, Vec<Value>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn from_values(values: &Values) -> Result<Self> {
        let mut st = SymbolTable::new();

        for mut value in values.clone().into_iter() {
            if let Some(file) = value.token.file() {
                st.files.insert(file);
            }

            if let Some(Type::App(types)) = value.typing.clone() {
                if types[0] == Type::Builtin {
                    let keyword = Keyword::from_str(&value.clone().name.unwrap())?;

                    match keyword {
                        Keyword::Import => {
                            let mut name_segs = Vec::new();

                            if let Some(path) = value.children[1].qualification.clone() {
                                name_segs.push(path);
                            }

                            name_segs.push(value.children[1].name.clone().unwrap());

                            let name = name_segs.join(".");

                            st.imp_paths.insert(name.clone());

                            st.imports
                                .entry(name)
                                .and_modify(|v| v.push(value.clone()))
                                .or_insert_with(|| vec![value]);
                        }
                        Keyword::Export => {
                            value = value.children[1].clone();

                            if value.children.len() > 1 {
                                let len = value.children.len();

                                for idx in 1..len {
                                    let child = value.children[idx].clone();

                                    let name = child.name.clone().unwrap();
                                    st.exp_defs.insert(name.clone());

                                    st.exports
                                        .entry(name)
                                        .and_modify(|v| v.push(value.clone()))
                                        .or_insert_with(|| vec![value.clone()]);
                                }
                            } else {
                                let name = value.name.clone().unwrap();
                                st.exp_defs.insert(name.clone());

                                st.exports
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            }
                        }
                        Keyword::Deftype => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_types.insert(name.clone());

                                st.scoped_types
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            } else {
                                st.def_types.insert(name.clone());

                                st.types
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            }

                            if name == "Main" {
                                if st.main_type.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate Main type".into(),
                                    }));
                                }

                                st.main_type = Some(value);
                            }
                        }
                        Keyword::Defsig => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_sigs.insert(name.clone());

                                st.scoped_sigs
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            } else {
                                st.def_sigs.insert(name.clone());

                                st.sigs
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            }

                            if name == "main" {
                                if st.main_sig.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main signature".into(),
                                    }));
                                }

                                st.main_sig = Some(value);
                            }
                        }
                        Keyword::Defprim => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_prims.insert(name.clone());

                                st.scoped_prims
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            } else {
                                st.def_prims.insert(name.clone());

                                st.prims
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            }
                        }
                        Keyword::Defsum => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_sums.insert(name.clone());

                                st.scoped_sums
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            } else {
                                st.def_sums.insert(name.clone());

                                st.sums
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            }
                        }
                        Keyword::Defprod => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_prods.insert(name.clone());
                                st.scoped_prods
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            } else {
                                st.def_prods.insert(name.clone());
                                st.prods
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            }
                        }
                        Keyword::Defun => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_funs.insert(name.clone());

                                st.scoped_funs
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            } else {
                                st.def_funs.insert(name.clone());

                                st.funs
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            }

                            if name == "main" {
                                if st.main_fun.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main function".into(),
                                    }));
                                }

                                st.main_fun = Some(value);
                            }
                        }
                        Keyword::Defattrs => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_def_attrs.insert(name.clone());

                                st.scoped_attrs
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            } else {
                                st.def_attrs.insert(name.clone());

                                st.attrs
                                    .entry(name.clone())
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value.clone()]);
                            }

                            if name == "main" {
                                if st.main_attrs.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main attributes".into(),
                                    }));
                                }

                                st.main_attrs = Some(value);
                            }
                        }
                        Keyword::Def => {
                            if value.children.len() != 3 {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid definition".into(),
                                }));
                            }

                            let name = value.children[1].name.clone().unwrap();
                            let name_value = value.children[2].clone();

                            let len = name_value.children.len();

                            if len >= 2 {
                                if name_value.children[0].name.is_none() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "expected a keyword".into(),
                                    }));
                                }

                                let kind = name_value.children[0].name.clone().unwrap();
                                let keyword = Keyword::from_string(kind)?;

                                match keyword {
                                    Keyword::Type => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_types.insert(name.clone());

                                            st.scoped_types
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        } else {
                                            st.def_types.insert(name.clone());

                                            st.types
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        }

                                        if name == "Main" {
                                            if st.main_type.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate Main type".into(),
                                                }));
                                            }

                                            st.main_type = Some(value);
                                        }
                                    }
                                    Keyword::Sig => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_sigs.insert(name.clone());

                                            st.scoped_sigs
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        } else {
                                            st.def_sigs.insert(name.clone());

                                            st.sigs
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        }

                                        if name == "main" {
                                            if st.main_sig.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main signature".into(),
                                                }));
                                            }

                                            st.main_sig = Some(value);
                                        }
                                    }
                                    Keyword::Prim => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_prims.insert(name.clone());

                                            st.scoped_prims
                                                .entry(name)
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value]);
                                        } else {
                                            st.def_prims.insert(name.clone());

                                            st.prims
                                                .entry(name)
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value]);
                                        }
                                    }
                                    Keyword::Sum => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_sums.insert(name.clone());

                                            st.scoped_sums
                                                .entry(name)
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value]);
                                        } else {
                                            st.def_sums.insert(name.clone());

                                            st.sums
                                                .entry(name)
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value]);
                                        }
                                    }
                                    Keyword::Prod => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_prods.insert(name.clone());

                                            st.scoped_prods
                                                .entry(name)
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value]);
                                        } else {
                                            st.def_prods.insert(name.clone());

                                            st.prods
                                                .entry(name)
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value]);
                                        }
                                    }
                                    Keyword::Fun => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_funs.insert(name.clone());

                                            st.scoped_funs
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        } else {
                                            st.def_funs.insert(name.clone());

                                            st.funs
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        }

                                        if name == "main" {
                                            if st.main_fun.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main function".into(),
                                                }));
                                            }

                                            st.main_fun = Some(value);
                                        }
                                    }
                                    Keyword::App => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_apps.insert(name.clone());

                                            st.scoped_apps
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        } else {
                                            st.def_apps.insert(name.clone());

                                            st.apps
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        }

                                        if name == "main" {
                                            if st.main_app.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main application".into(),
                                                }));
                                            }

                                            st.main_app = Some(value);
                                        }
                                    }
                                    Keyword::Attrs => {
                                        if !value.scope_path.is_empty() {
                                            st.scoped_def_attrs.insert(name.clone());

                                            st.scoped_attrs
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        } else {
                                            st.def_attrs.insert(name.clone());

                                            st.attrs
                                                .entry(name.clone())
                                                .and_modify(|v| v.push(value.clone()))
                                                .or_insert_with(|| vec![value.clone()]);
                                        }

                                        if name == "main" {
                                            if st.main_attrs.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main attributes".into(),
                                                }));
                                            }

                                            st.main_attrs = Some(value);
                                        }
                                    }
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: name_value.token.loc(),
                                            desc: "unexpected keyword".into(),
                                        }));
                                    }
                                }
                            } else if len == 1 {
                                let name = value.name.clone().unwrap();

                                if !value.scope_path.is_empty() {
                                    st.scoped_def_prims.insert(name.clone());

                                    st.scoped_prims
                                        .entry(name)
                                        .and_modify(|v| v.push(value.clone()))
                                        .or_insert_with(|| vec![value]);
                                } else {
                                    st.def_prims.insert(name.clone());

                                    st.prims
                                        .entry(name)
                                        .and_modify(|v| v.push(value.clone()))
                                        .or_insert_with(|| vec![value]);
                                }
                            } else {
                                return Err(Error::Semantic(SemanticError {
                                    loc: name_value.token.loc(),
                                    desc: "invalid definition".into(),
                                }));
                            }
                        }
                        _ if value.prim.is_none() && value.children.len() > 1 => {
                            let name = value.children[1].name.clone().unwrap();

                            if !value.scope_path.is_empty() {
                                st.scoped_apps
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            } else {
                                st.apps
                                    .entry(name)
                                    .and_modify(|v| v.push(value.clone()))
                                    .or_insert_with(|| vec![value]);
                            }
                        }
                        _ => {}
                    }
                } else if value.prim.is_none() && value.children.len() > 1 {
                    let name = value.children[0].name.clone().unwrap();

                    if !value.scope_path.is_empty() {
                        st.scoped_apps
                            .entry(name)
                            .and_modify(|v| v.push(value.clone()))
                            .or_insert_with(|| vec![value]);
                    } else {
                        st.apps
                            .entry(name)
                            .and_modify(|v| v.push(value.clone()))
                            .or_insert_with(|| vec![value]);
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

        let mut s = "(deftype RGB (Prod UInt UInt UInt))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_types.len(), 1);
        assert!(st.def_types.contains("RGB"));
        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));

        s = "(def RGB (type (Prod UInt UInt UInt)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_types.len(), 1);
        assert!(st.def_types.contains("RGB"));
        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));
    }

    #[test]
    fn symbol_table_prims() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defprim i 0)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_prims.len(), 1);
        assert!(st.def_prims.contains("i"));
        assert_eq!(st.prims.len(), 1);
        assert!(st.prims.contains_key("i"));

        s = "(def i (prim 0))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_prims.len(), 1);
        assert!(st.def_prims.contains("i"));
        assert_eq!(st.prims.len(), 1);
        assert!(st.prims.contains_key("i"));
    }

    #[test]
    fn symbol_table_sums() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defsum predicate true)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_sums.len(), 1);
        assert!(st.def_sums.contains("predicate"));
        assert_eq!(st.sums.len(), 1);
        assert!(st.sums.contains_key("predicate"));

        s = "(def predicate (sum true))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_sums.len(), 1);
        assert!(st.def_sums.contains("predicate"));
        assert_eq!(st.sums.len(), 1);
        assert!(st.sums.contains_key("predicate"));
    }

    #[test]
    fn symbol_table_prods() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defprod result 1 ())";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_prods.len(), 1);
        assert!(st.def_prods.contains("result"));
        assert_eq!(st.prods.len(), 1);
        assert!(st.prods.contains_key("result"));

        s = "(def result (prod 1 ()))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_prods.len(), 1);
        assert!(st.def_prods.contains("result"));
        assert_eq!(st.prods.len(), 1);
        assert!(st.prods.contains_key("result"));
    }

    #[test]
    fn symbol_table_funs() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defun rShift x i (>> x i))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_funs.len(), 1);
        assert!(st.def_funs.contains("rShift"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("rShift"));

        s = "(def rShift (fun x i (>> x i)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_funs.len(), 1);
        assert!(st.def_funs.contains("rShift"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("rShift"));
    }

    #[test]
    fn symbol_table_apps() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(def res (app f x y z))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_apps.len(), 1);
        assert!(st.def_apps.contains("res"));
        assert_eq!(st.apps.len(), 1);
        assert!(st.apps.contains_key("res"));

        s = "(f x y z)";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_apps.len(), 0);
        assert!(!st.def_apps.contains("f"));
        assert_eq!(st.apps.len(), 1);
        assert!(st.apps.contains_key("f"));
    }

    #[test]
    fn symbol_table_attrs() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defattrs sum attr1 attr2 attr3)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_attrs.len(), 1);
        assert!(st.def_attrs.contains("sum"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));

        s = "(def sum (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_attrs.len(), 1);
        assert!(st.def_attrs.contains("sum"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));
    }

    #[test]
    fn symbol_table_main() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defsig main (Fun IO IO))\n(defun main io (id io))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.is_some());
        assert_eq!(st.def_sigs.len(), 1);
        assert!(st.def_sigs.contains("main"));
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.def_funs.len(), 1);
        assert!(st.def_funs.contains("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));

        s = "(def main (sig (Fun IO IO)))\n(def main (fun io (id io)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

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
