use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::syntax::is_type_symbol;
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
                                if form.kind != FormKind::ProdDef {
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
                        if self.imported_qualifiers.get(&qualifier).is_some() {
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
                            if self.imported_types.get(qualified_name).is_some() {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "reimported type".into(),
                                }));
                            }

                            self.imported_types
                                .insert(qualified_name.into(), module.clone());
                        } else {
                            if self.imported_values.get(qualified_name).is_some() {
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
                FormKind::ExportDefs => {}
                FormKind::Def => {}
                FormKind::TypeDef => {}
                FormKind::SigDef => {}
                FormKind::AttrsDef => {}
                FormKind::PrimDef => {}
                FormKind::SumDef => {}
                FormKind::ProdDef => {}
                FormKind::FunDef => {}
                FormKind::AppDef => {}
                FormKind::FunApp => {}
                FormKind::TypeApp => {}
                FormKind::LetScope => {}
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
}
