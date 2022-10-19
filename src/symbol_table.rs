use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::syntax::Keyword;
use crate::typing::Type;
use crate::value::Value;
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub values: Values,

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

    pub imports: BTreeMap<String, Vec<usize>>,
    pub exports: BTreeMap<String, Vec<usize>>,
    pub types: BTreeMap<String, Vec<usize>>,
    pub prims: BTreeMap<String, Vec<usize>>,
    pub sums: BTreeMap<String, Vec<usize>>,
    pub prods: BTreeMap<String, Vec<usize>>,
    pub sigs: BTreeMap<String, Vec<usize>>,
    pub funs: BTreeMap<String, Vec<usize>>,
    pub apps: BTreeMap<String, Vec<usize>>,
    pub attrs: BTreeMap<String, Vec<usize>>,

    pub main_type: Option<usize>,
    pub main_sig: Option<usize>,
    pub main_fun: Option<usize>,
    pub main_app: Option<usize>,
    pub main_fun_attrs: Option<usize>,
    pub main_type_attrs: Option<usize>,

    pub scoped_def_types: BTreeSet<String>,
    pub scoped_def_prims: BTreeSet<String>,
    pub scoped_def_sums: BTreeSet<String>,
    pub scoped_def_prods: BTreeSet<String>,
    pub scoped_def_sigs: BTreeSet<String>,
    pub scoped_def_funs: BTreeSet<String>,
    pub scoped_def_apps: BTreeSet<String>,
    pub scoped_def_attrs: BTreeSet<String>,

    pub scoped_types: BTreeMap<String, Vec<usize>>,
    pub scoped_prims: BTreeMap<String, Vec<usize>>,
    pub scoped_sums: BTreeMap<String, Vec<usize>>,
    pub scoped_prods: BTreeMap<String, Vec<usize>>,
    pub scoped_sigs: BTreeMap<String, Vec<usize>>,
    pub scoped_funs: BTreeMap<String, Vec<usize>>,
    pub scoped_apps: BTreeMap<String, Vec<usize>>,
    pub scoped_attrs: BTreeMap<String, Vec<usize>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn add_value(&mut self, value_ref: &Value) -> Result<()> {
        let mut value = value_ref.clone();

        self.values.push(value.clone());
        let value_idx = self.values.len() - 1;

        if let Some(file) = value.token.file() {
            self.files.insert(file);
        }

        if let Some(Type::App(types)) = value.typing.clone() {
            if types[0] == Type::Builtin {
                let name = value.clone().name.unwrap();
                let keyword = Keyword::from_str(&name)?;

                match keyword {
                    Keyword::Import => {
                        let mut name_segs = Vec::new();

                        if let Some(path) = value.children[1].qualification.clone() {
                            name_segs.push(path);
                        }

                        name_segs.push(value.children[1].name.clone().unwrap());

                        let name = name_segs.join(".");

                        self.imp_paths.insert(name.clone());

                        self.imports
                            .entry(name)
                            .and_modify(|v| v.push(value_idx))
                            .or_insert_with(|| vec![value_idx]);
                    }
                    Keyword::Export => {
                        value = value.children[1].clone();

                        if value.children.len() > 1 {
                            let len = value.children.len();

                            for idx in 1..len {
                                let child = value.children[idx].clone();

                                let name = child.name.clone().unwrap();
                                self.exp_defs.insert(name.clone());

                                self.exports
                                    .entry(name)
                                    .and_modify(|v| v.push(value_idx))
                                    .or_insert_with(|| vec![value_idx]);
                            }
                        } else {
                            let name = value.name.clone().unwrap();
                            self.exp_defs.insert(name.clone());

                            self.exports
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        }
                    }
                    Keyword::Deftype => {
                        let name = value.children[1].name.clone().unwrap();

                        if !value.scope.is_tpl() {
                            if name == "Main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped Main type".into(),
                                }));
                            }

                            self.scoped_def_types.insert(name.clone());

                            self.scoped_types
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_types.insert(name.clone());

                            self.types
                                .entry(name.clone())
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);

                            if name == "Main" {
                                if self.main_type.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate Main type".into(),
                                    }));
                                }

                                self.main_type = Some(value_idx);
                            }
                        }
                    }
                    Keyword::Defsig => {
                        let name = value.children[1].name.clone().unwrap();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main signature".into(),
                                }));
                            }

                            self.scoped_def_sigs.insert(name.clone());

                            self.scoped_sigs
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_sigs.insert(name.clone());

                            self.sigs
                                .entry(name.clone())
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);

                            if name == "main" {
                                if self.main_sig.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main signature".into(),
                                    }));
                                }

                                self.main_sig = Some(value_idx);
                            }
                        }
                    }
                    Keyword::Defprim => {
                        let name = value.children[1].name.clone().unwrap();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_def_prims.insert(name.clone());

                            self.scoped_prims
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_prims.insert(name.clone());

                            self.prims
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        }
                    }
                    Keyword::Defsum => {
                        let name = value.children[1].name.clone().unwrap();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_def_sums.insert(name.clone());

                            self.scoped_sums
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_sums.insert(name.clone());

                            self.sums
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        }
                    }
                    Keyword::Defprod => {
                        let name = value.children[1].name.clone().unwrap();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_def_prods.insert(name.clone());
                            self.scoped_prods
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_prods.insert(name.clone());
                            self.prods
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        }
                    }
                    Keyword::Defun => {
                        let name = value.children[1].name.clone().unwrap();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main function".into(),
                                }));
                            }

                            self.scoped_def_funs.insert(name.clone());

                            self.scoped_funs
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_funs.insert(name.clone());

                            self.funs
                                .entry(name.clone())
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);

                            if name == "main" {
                                if self.main_fun.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main function".into(),
                                    }));
                                }

                                self.main_fun = Some(value_idx);
                            }
                        }
                    }
                    Keyword::Defattrs => {
                        let name = value.children[1].name.clone().unwrap();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main attributes".into(),
                                }));
                            }

                            self.scoped_def_attrs.insert(name.clone());

                            self.scoped_attrs
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.def_attrs.insert(name.clone());

                            self.attrs
                                .entry(name.clone())
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);

                            if name == "main" {
                                if self.main_fun_attrs.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main function attributes".into(),
                                    }));
                                }

                                self.main_fun_attrs = Some(value_idx);
                            } else if name == "Main" {
                                if self.main_type_attrs.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate Main type attributes".into(),
                                    }));
                                }

                                self.main_type_attrs = Some(value_idx);
                            }
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
                                    if !value.scope.is_tpl() {
                                        if name == "Main" {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: name_value.token.loc(),
                                                desc: "invalid scoped Main type".into(),
                                            }));
                                        }

                                        self.scoped_def_types.insert(name.clone());

                                        self.scoped_types
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_types.insert(name.clone());

                                        self.types
                                            .entry(name.clone())
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);

                                        if name == "Main" {
                                            if self.main_type.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate Main type".into(),
                                                }));
                                            }

                                            self.main_type = Some(value_idx);
                                        }
                                    }
                                }
                                Keyword::Sig => {
                                    if !value.scope.is_tpl() {
                                        if name == "main" {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: name_value.token.loc(),
                                                desc: "invalid scoped main signature".into(),
                                            }));
                                        }

                                        self.scoped_def_sigs.insert(name.clone());

                                        self.scoped_sigs
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_sigs.insert(name.clone());

                                        self.sigs
                                            .entry(name.clone())
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);

                                        if name == "main" {
                                            if self.main_sig.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main signature".into(),
                                                }));
                                            }

                                            self.main_sig = Some(value_idx);
                                        }
                                    }
                                }
                                Keyword::Prim => {
                                    if name == "main" {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: value.token.loc(),
                                            desc: "invalid main".into(),
                                        }));
                                    }

                                    if !value.scope.is_tpl() {
                                        self.scoped_def_prims.insert(name.clone());

                                        self.scoped_prims
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_prims.insert(name.clone());

                                        self.prims
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    }
                                }
                                Keyword::Sum => {
                                    if name == "main" {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: value.token.loc(),
                                            desc: "invalid main".into(),
                                        }));
                                    }

                                    if !value.scope.is_tpl() {
                                        self.scoped_def_sums.insert(name.clone());

                                        self.scoped_sums
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_sums.insert(name.clone());

                                        self.sums
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    }
                                }
                                Keyword::Prod => {
                                    if name == "main" {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: value.token.loc(),
                                            desc: "invalid main".into(),
                                        }));
                                    }

                                    if !value.scope.is_tpl() {
                                        self.scoped_def_prods.insert(name.clone());

                                        self.scoped_prods
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_prods.insert(name.clone());

                                        self.prods
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    }
                                }
                                Keyword::Fun => {
                                    if !value.scope.is_tpl() {
                                        if name == "main" {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: name_value.token.loc(),
                                                desc: "invalid scoped main function".into(),
                                            }));
                                        }

                                        self.scoped_def_funs.insert(name.clone());

                                        self.scoped_funs
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_funs.insert(name.clone());

                                        self.funs
                                            .entry(name.clone())
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);

                                        if name == "main" {
                                            if self.main_fun.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main function".into(),
                                                }));
                                            }

                                            self.main_fun = Some(value_idx);
                                        }
                                    }
                                }
                                Keyword::App => {
                                    if !value.scope.is_tpl() {
                                        if name == "main" {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: name_value.token.loc(),
                                                desc: "invalid scoped main application".into(),
                                            }));
                                        }

                                        self.scoped_def_apps.insert(name.clone());

                                        self.scoped_apps
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_apps.insert(name.clone());

                                        self.apps
                                            .entry(name.clone())
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);

                                        if name == "main" {
                                            if self.main_app.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main application".into(),
                                                }));
                                            }

                                            self.main_app = Some(value_idx);
                                        }
                                    }
                                }
                                Keyword::Attrs => {
                                    if !value.scope.is_tpl() {
                                        if name == "main" {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: name_value.token.loc(),
                                                desc: "invalid scoped main attributes".into(),
                                            }));
                                        }

                                        self.scoped_def_attrs.insert(name.clone());

                                        self.scoped_attrs
                                            .entry(name)
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);
                                    } else {
                                        self.def_attrs.insert(name.clone());

                                        self.attrs
                                            .entry(name.clone())
                                            .and_modify(|v| v.push(value_idx))
                                            .or_insert_with(|| vec![value_idx]);

                                        if name == "main" {
                                            if self.main_fun_attrs.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main function attributes"
                                                        .into(),
                                                }));
                                            }

                                            self.main_fun_attrs = Some(value_idx);
                                        } else if name == "Main" {
                                            if self.main_type_attrs.is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate Main type attributes".into(),
                                                }));
                                            }

                                            self.main_type_attrs = Some(value_idx);
                                        }
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

                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid main".into(),
                                }));
                            }

                            if !value.scope.is_tpl() {
                                self.scoped_def_prims.insert(name.clone());

                                self.scoped_prims
                                    .entry(name)
                                    .and_modify(|v| v.push(value_idx))
                                    .or_insert_with(|| vec![value_idx]);
                            } else {
                                self.def_prims.insert(name.clone());

                                self.prims
                                    .entry(name)
                                    .and_modify(|v| v.push(value_idx))
                                    .or_insert_with(|| vec![value_idx]);
                            }
                        } else {
                            return Err(Error::Semantic(SemanticError {
                                loc: name_value.token.loc(),
                                desc: "invalid definition".into(),
                            }));
                        }
                    }
                    _ => {
                        let name = if value.children[1].name.is_some() {
                            value.children[1].name.clone().unwrap()
                        } else {
                            "_".into()
                        };

                        if value.prim.is_some() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main primitive".into(),
                                }));
                            }

                            if !value.scope.is_tpl() {
                                self.scoped_prims
                                    .entry(name)
                                    .and_modify(|v| v.push(value_idx))
                                    .or_insert_with(|| vec![value_idx]);
                            } else {
                                self.prims
                                    .entry(name)
                                    .and_modify(|v| v.push(value_idx))
                                    .or_insert_with(|| vec![value_idx]);
                            }
                        } else if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main application".into(),
                                }));
                            }

                            self.scoped_apps
                                .entry(name)
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);
                        } else {
                            self.apps
                                .entry(name.clone())
                                .and_modify(|v| v.push(value_idx))
                                .or_insert_with(|| vec![value_idx]);

                            if name == "main" {
                                if self.main_app.is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main application".into(),
                                    }));
                                }

                                self.main_app = Some(value_idx);
                            }
                        }
                    }
                }
            } else if value.prim.is_none() {
                let name = value.children[0].name.clone().unwrap();

                if !value.scope.is_tpl() {
                    if name == "main" {
                        return Err(Error::Semantic(SemanticError {
                            loc: value.token.loc(),
                            desc: "invalid scoped main application".into(),
                        }));
                    }

                    self.scoped_apps
                        .entry(name)
                        .and_modify(|v| v.push(value_idx))
                        .or_insert_with(|| vec![value_idx]);
                } else {
                    self.apps
                        .entry(name.clone())
                        .and_modify(|v| v.push(value_idx))
                        .or_insert_with(|| vec![value_idx]);

                    if name == "main" {
                        if self.main_app.is_some() {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "duplicate main application".into(),
                            }));
                        }

                        self.main_app = Some(value_idx);
                    }
                }
            }
        }

        for child_value in value.children.iter() {
            self.add_value(child_value)?;
        }

        Ok(())
    }

    pub fn from_values(values: &Values) -> Result<Self> {
        let mut st = SymbolTable::new();

        for value in values.clone().into_iter() {
            st.add_value(&value)?;
        }

        Ok(st)
    }

    pub fn position(&self, value: Value) -> Option<usize> {
        self.values.position(value)
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

        s = "(f x y (g z))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.scoped_def_apps.len(), 0);
        assert!(!st.scoped_def_apps.contains("g"));
        assert_eq!(st.scoped_apps.len(), 1);
        assert!(st.scoped_apps.contains_key("g"));
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

        s = "(def Main (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        let value = values[0].clone();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_attrs.len(), 1);
        assert!(st.def_attrs.contains("Main"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("Main"));
        assert_eq!(st.main_type_attrs, st.position(value));
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
