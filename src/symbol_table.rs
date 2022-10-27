use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::syntax::is_type_symbol;
use crate::syntax::Keyword;
use crate::syntax::{symbol_name, symbol_qualifier, symbol_with_qualifier};
use crate::value::{FormKind, Value};
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub file: String,
    pub values: Values,

    // module -> set(value_idx)
    pub imported_modules: BTreeMap<String, BTreeSet<usize>>,
    // qualifier -> module
    pub imported_qualifiers: BTreeMap<String, String>,
    // name -> module
    pub imported_types: BTreeMap<String, String>,
    pub imported_values: BTreeMap<String, String>,

    // name -> value_idx
    pub exported_values: BTreeMap<String, usize>,
    pub exported_types: BTreeMap<String, usize>,

    // name -> value_idx
    pub type_attrs_defs: BTreeMap<String, usize>,
    pub fun_attrs_defs: BTreeMap<String, usize>,
    // name -> set(attr)
    pub type_attrs: BTreeMap<String, BTreeSet<String>>,
    pub fun_attrs: BTreeMap<String, BTreeSet<String>>,

    // name -> value_idx
    pub type_defs: BTreeMap<String, usize>,
    pub sig_defs: BTreeMap<String, usize>,
    pub prim_defs: BTreeMap<String, usize>,
    pub sum_defs: BTreeMap<String, usize>,
    pub prod_defs: BTreeMap<String, usize>,
    pub fun_defs: BTreeMap<String, usize>,
    pub app_defs: BTreeMap<String, usize>,

    pub main_type_attrs: Option<usize>,
    pub main_fun_attrs: Option<usize>,
    pub main_type: Option<usize>,
    pub main_sig: Option<usize>,
    pub main_fun: Option<usize>,
    pub main_app: Option<usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable::default()
    }

    pub fn add_value(&mut self, value: Value) -> Result<()> {
        let value_file = value.file();
        let value_loc = value.loc();

        if self.file != value_file {
            if self.file.is_empty() && !value_file.is_empty() {
                self.file = value_file;
            } else {
                return Err(Error::Semantic(SemanticError {
                    loc: value_loc,
                    desc: "expected a value from the same file".into(),
                }));
            }
        }

        let tpl_idx = self.values.len();
        self.values.push(value.clone());

        match value {
            Value::Form(form) => match form.kind {
                FormKind::Empty => {}
                FormKind::ImportDefs => {
                    let module = form.values[1].to_string();

                    let mut qualified_names: Vec<String> = Vec::new();

                    let mut qualifier = String::new();

                    if form.values.len() > 3 {
                        match form.values[3].clone() {
                            Value::Symbol(symbol) => {
                                qualifier = symbol.to_string();
                            }
                            _ => {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "expected a qualifier symbol".into(),
                                }));
                            }
                        };
                    }

                    if form.values.len() > 2 {
                        match form.values[2].clone() {
                            Value::Form(form) => {
                                if form.kind != FormKind::AnonProd {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected a product of imported definitions".into(),
                                    }));
                                }

                                for imported_value in form.values[1..].iter() {
                                    let name = imported_value.to_string();

                                    if !symbol_qualifier(&name).is_empty() {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected an unqualified name".into(),
                                        }));
                                    }

                                    let qualified_name = symbol_with_qualifier(&name, &qualifier);

                                    qualified_names.push(qualified_name);
                                }
                            }
                            _ => {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "expected a product of imported definitions".into(),
                                }));
                            }
                        }
                    }

                    self.imported_modules
                        .entry(module.clone())
                        .and_modify(|set| {
                            set.insert(tpl_idx);
                        })
                        .or_insert_with(|| {
                            let mut set = BTreeSet::new();
                            set.insert(tpl_idx);
                            set
                        });

                    if !qualifier.is_empty() {
                        if self.imported_qualifiers.contains_key(&qualifier) {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a value from the same file".into(),
                            }));
                        }

                        self.imported_qualifiers.insert(qualifier, module.clone());
                    }

                    for qualified_name in qualified_names.iter() {
                        let name = symbol_name(&qualified_name);

                        if is_type_symbol(&name) {
                            if self.imported_types.contains_key(qualified_name) {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "reimported type".into(),
                                }));
                            }

                            self.imported_types
                                .insert(qualified_name.into(), module.clone());
                        } else {
                            if self.imported_values.contains_key(qualified_name) {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "reimported type".into(),
                                }));
                            }

                            self.imported_values
                                .insert(qualified_name.into(), module.clone());
                        }
                    }
                }
                FormKind::ExportDefs => match form.values[1].clone() {
                    Value::Symbol(symbol) => {
                        let name = symbol.to_string();

                        if !symbol_qualifier(&name).is_empty() {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected an unqualified symbol".into(),
                            }));
                        }

                        if is_type_symbol(&name) {
                            if self.exported_types.contains_key(&name) {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "reexported definition".into(),
                                }));
                            }

                            self.exported_types.insert(name, tpl_idx);
                        } else {
                            if self.exported_values.contains_key(&name) {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "reexported definition".into(),
                                }));
                            }

                            self.exported_values.insert(name, tpl_idx);
                        }
                    }
                    Value::Form(form) => {
                        if form.kind != FormKind::AnonProd {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a product of exported definitions".into(),
                            }));
                        }

                        for exported_value in form.values[1..].iter() {
                            let name = exported_value.to_string();

                            if !symbol_qualifier(&name).is_empty() {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "expected an unqualified symbol".into(),
                                }));
                            }

                            if is_type_symbol(&name) {
                                if self.exported_types.contains_key(&name) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "reexported definition".into(),
                                    }));
                                }

                                self.exported_types.insert(name, tpl_idx);
                            } else {
                                if self.exported_values.contains_key(&name) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "reexported definition".into(),
                                    }));
                                }

                                self.exported_values.insert(name, tpl_idx);
                            }
                        }
                    }
                    _ => {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected a form or a symbol".into(),
                        }));
                    }
                },
                FormKind::DefType => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.type_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined type".into(),
                        }));
                    }

                    self.type_defs.insert(name, tpl_idx);
                }
                FormKind::DefSig => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.sig_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined signature".into(),
                        }));
                    }

                    self.sig_defs.insert(name, tpl_idx);
                }
                FormKind::DefPrim => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.prim_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined primitive".into(),
                        }));
                    }

                    self.prim_defs.insert(name, tpl_idx);
                }
                FormKind::DefSum => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.sum_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined sum".into(),
                        }));
                    }

                    self.sum_defs.insert(name, tpl_idx);
                }
                FormKind::DefProd => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.prod_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined prod".into(),
                        }));
                    }

                    self.prod_defs.insert(name, tpl_idx);
                }
                FormKind::DefFun => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.fun_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined function".into(),
                        }));
                    }

                    self.fun_defs.insert(name, tpl_idx);
                }
                FormKind::DefApp => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::from_str(&name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };
                            }

                            name
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if self.app_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined function application".into(),
                        }));
                    }

                    self.app_defs.insert(name, tpl_idx);
                }
                FormKind::DefAttrs => {
                    #[allow(unused_assignments)] 
                    let mut name = String::new();
                    let mut attributes: Vec<String> = Vec::new();

                    match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let tmp_name = symbol.to_string();

                            if Keyword::from_str(&tmp_name).is_ok() {
                                name = match form.values[2].clone() {
                                    Value::Symbol(symbol) => symbol.to_string(),
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected a symbol".into(),
                                        }));
                                    }
                                };

                                if form.values.len() < 4 {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected at least an attribute".into(),
                                    }));
                                }

                                for value in form.values[3..].iter() {
                                    match value {
                                        Value::Symbol(symbol) => {
                                            attributes.push(symbol.to_string());
                                        }
                                        _ => {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: value.loc(),
                                                desc: "expected a symbol".into(),
                                            }));
                                        }
                                    }
                                }
                            } else {
                                name = tmp_name;

                                match form.values[2].clone() {
                                    Value::Form(form) => {
                                        if form.kind != FormKind::AnonAttrs {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: form.loc(),
                                                desc: "expected an attributes form".into(),
                                            }));
                                        }

                                        if form.len() < 2 {
                                            return Err(Error::Semantic(SemanticError {
                                                loc: form.loc(),
                                                desc: "expected at least an attribute".into(),
                                            }));
                                        }

                                        for value in form.values[1..].iter() {
                                            match value {
                                                Value::Symbol(symbol) => {
                                                    attributes.push(symbol.to_string());
                                                }
                                                _ => {
                                                    return Err(Error::Semantic(SemanticError {
                                                        loc: value.loc(),
                                                        desc: "expected a symbol".into(),
                                                    }));
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected an attributes form".into(),
                                        }));
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    };

                    if is_type_symbol(&name) {
                        if self.type_attrs_defs.contains_key(&name) {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined type attributes".into(),
                            }));
                        }

                        self.type_attrs_defs.insert(name.clone(), tpl_idx);

                        for attribute in attributes.iter() {
                            self.type_attrs
                                .entry(name.to_owned())
                                .and_modify(|set| {
                                    set.insert(attribute.clone());
                                })
                                .or_insert_with(|| {
                                    let mut set = BTreeSet::new();
                                    set.insert(attribute.to_owned());
                                    set
                                });
                        }
                    } else {
                        if self.fun_attrs_defs.contains_key(&name) {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined function attributes".into(),
                            }));
                        }

                        self.fun_attrs_defs.insert(name.clone(), tpl_idx);

                        for attribute in attributes.iter() {
                            self.fun_attrs
                                .entry(name.to_owned())
                                .and_modify(|set| {
                                    set.insert(attribute.clone());
                                })
                                .or_insert_with(|| {
                                    let mut set = BTreeSet::new();
                                    set.insert(attribute.to_owned());
                                    set
                                });
                        }
                    }
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: "expected an import or an export or a definition or a 'main' function application at top-level".into(),
                    }));
                }
            },
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: value.loc(),
                    desc: "expected a form".into(),
                }));
            }
        }

        Ok(())
    }

    pub fn add_values(&mut self, values: Values) -> Result<()> {
        for value in values.into_iter() {
            self.add_value(value)?;
        }

        Ok(())
    }

    pub fn from_values(values: Values) -> Result<SymbolTable> {
        let mut symbol_table = SymbolTable::new();

        symbol_table.add_values(values)?;

        Ok(symbol_table)
    }

    pub fn from_str(s: &str) -> Result<SymbolTable> {
        let values = Values::from_str(s)?;

        SymbolTable::from_values(values)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SymbolTable> {
        SymbolTable::from_str(&fs::read_to_string(path)?)
    }
}

pub struct GlobalSymbolTable {
    pub files: BTreeSet<String>,

    // file -> symbol_table
    pub tables: BTreeMap<String, SymbolTable>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn symbol_table_imports() {
        use super::SymbolTable;
        use crate::syntax::EMPTY;
        use crate::values::Values;

        let s = "(import moduleX (prod a b c D) x)";

        let values = Values::from_str(s).unwrap();

        let res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        let symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.imported_modules.len(), 1);
        assert!(symbol_table
            .imported_modules
            .get("moduleX")
            .unwrap()
            .contains(&0));
        assert_eq!(symbol_table.imported_qualifiers.len(), 1);
        assert_eq!(
            symbol_table.imported_qualifiers.get("x"),
            Some(&"moduleX".into())
        );
        assert_eq!(symbol_table.imported_types.len(), 1);
        assert_eq!(
            symbol_table.imported_types.get("x.D"),
            Some(&"moduleX".into())
        );
        assert_eq!(symbol_table.imported_values.len(), 3);
        assert_eq!(
            symbol_table.imported_values.get("x.a"),
            Some(&"moduleX".into())
        );
    }

    #[test]
    fn symbol_table_exports() {
        use super::SymbolTable;
        use crate::syntax::EMPTY;
        use crate::values::Values;

        let mut s = "(export (prod a b c D))";

        let mut values = Values::from_str(s).unwrap();

        let mut res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        let mut symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.exported_types.len(), 1);
        assert_eq!(symbol_table.exported_types.get("D"), Some(&0));
        assert_eq!(symbol_table.exported_values.len(), 3);
        assert_eq!(symbol_table.exported_values.get("a"), Some(&0));

        s = "(export X)";

        values = Values::from_str(s).unwrap();

        res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.exported_types.len(), 1);
        assert_eq!(symbol_table.exported_types.get("X"), Some(&0));
        assert_eq!(symbol_table.exported_values.len(), 0);
    }

    #[test]
    fn symbol_table_definitions() {
        use super::SymbolTable;
        use crate::syntax::EMPTY;
        use crate::values::Values;

        let mut s = "(def X (type (Prod String UInt Float)))";

        let mut values = Values::from_str(s).unwrap();

        let mut res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        let mut symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.type_defs.len(), 1);
        assert_eq!(symbol_table.type_defs.get("X"), Some(&0));

        s = "(def f (fun x y (+ x y)))";

        values = Values::from_str(s).unwrap();

        res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.fun_defs.len(), 1);
        assert_eq!(symbol_table.fun_defs.get("f"), Some(&0));

        s = "(def fun f x y (+ x y))";

        values = Values::from_str(s).unwrap();

        res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.fun_defs.len(), 1);
        assert_eq!(symbol_table.fun_defs.get("f"), Some(&0));

        s = "(def attrs f attr1 attr2 attr3)";

        values = Values::from_str(s).unwrap();

        res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.fun_attrs_defs.len(), 1);
        assert_eq!(symbol_table.fun_attrs_defs.get("f"), Some(&0));
        assert_eq!(symbol_table.fun_attrs.len(), 1);
        assert_eq!(symbol_table.fun_attrs.get("f").unwrap().len(), 3);
        assert!(symbol_table.fun_attrs.get("f").unwrap().contains("attr1"));
        assert!(symbol_table.fun_attrs.get("f").unwrap().contains("attr2"));
        assert!(symbol_table.fun_attrs.get("f").unwrap().contains("attr3"));

        s = "(def F (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        res = SymbolTable::from_values(values);

        assert!(res.is_ok());

        symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert_eq!(symbol_table.type_attrs_defs.len(), 1);
        assert_eq!(symbol_table.type_attrs_defs.get("F"), Some(&0));
        assert_eq!(symbol_table.type_attrs.len(), 1);
        assert_eq!(symbol_table.type_attrs.get("F").unwrap().len(), 3);
        assert!(symbol_table.type_attrs.get("F").unwrap().contains("attr1"));
        assert!(symbol_table.type_attrs.get("F").unwrap().contains("attr2"));
        assert!(symbol_table.type_attrs.get("F").unwrap().contains("attr3"));
    }
}
