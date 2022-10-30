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

    pub defs: BTreeMap<String, usize>,

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

    pub fn is_defined(&self, name: &str) -> bool {
        self.defs.contains_key(name)
    }

    pub fn is_undefined(&self, name: &str) -> bool {
        !self.is_defined(name)
    }

    pub fn find_index(&self, name: &str) -> Option<usize> {
        self.defs.get(name).map(|idx| idx.to_owned())
    }

    pub fn find_value(&self, name: &str) -> Option<Value> {
        self.find_index(name).map(|idx| self.values[idx].clone())
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
                        if self.defs.contains_key(qualified_name) {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined value".into(),
                            }));
                        } else {
                            self.defs.insert(qualified_name.clone(), tpl_idx);
                        }

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

                            if Keyword::is(&name) {
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

                    if self.defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined value".into(),
                        }));
                    } else {
                        self.defs.insert(name.clone(), tpl_idx);
                    }

                    if self.type_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined type".into(),
                        }));
                    }

                    self.type_defs.insert(name.clone(), tpl_idx);

                    if name == "Main" {
                        if self.main_type.is_some() {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined 'Main' type".into(),
                            }));
                        }

                        self.main_type = Some(tpl_idx);
                    }
                }
                FormKind::DefSig => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::is(&name) {
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

                    self.sig_defs.insert(name.clone(), tpl_idx);

                    if name == "main" {
                        if self.main_sig.is_some() {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined 'main' signature".into(),
                            }));
                        }

                        self.main_sig = Some(tpl_idx);
                    }
                }
                FormKind::DefPrim => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::is(&name) {
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

                    if self.defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined value".into(),
                        }));
                    } else {
                        self.defs.insert(name.clone(), tpl_idx);
                    }

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

                            if Keyword::is(&name) {
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

                    if self.defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined value".into(),
                        }));
                    } else {
                        self.defs.insert(name.clone(), tpl_idx);
                    }

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

                            if Keyword::is(&name) {
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

                    if self.defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined value".into(),
                        }));
                    } else {
                        self.defs.insert(name.clone(), tpl_idx);
                    }

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

                            if Keyword::is(&name) {
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

                    if self.defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined value".into(),
                        }));
                    } else {
                        self.defs.insert(name.clone(), tpl_idx);
                    }

                    if self.fun_defs.contains_key(&name) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "redefined function".into(),
                        }));
                    }

                    self.fun_defs.insert(name.clone(), tpl_idx);

                    if name == "main" {
                        if self.main_fun.is_some() {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined 'main' function".into(),
                            }));
                        }

                        self.main_fun = Some(tpl_idx);
                    }
                }
                FormKind::DefApp => {
                    let name = match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let mut name = symbol.to_string();

                            if Keyword::is(&name) {
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

                    self.app_defs.insert(name.clone(), tpl_idx);

                    if name == "main" {
                        if self.main_app.is_some() {
                            return Err(Error::Semantic(SemanticError {
                                loc: form.loc(),
                                desc: "redefined 'main' application".into(),
                            }));
                        }

                        self.main_app = Some(tpl_idx);
                    }
                }
                FormKind::DefAttrs => {
                    #[allow(unused_assignments)]
                    let mut name = String::new();

                    let mut attributes: Vec<String> = Vec::new();

                    match form.values[1].clone() {
                        Value::Symbol(symbol) => {
                            let tmp_name = symbol.to_string();

                            if Keyword::is(&tmp_name) {
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
                                .entry(name.clone())
                                .and_modify(|set| {
                                    set.insert(attribute.clone());
                                })
                                .or_insert_with(|| {
                                    let mut set = BTreeSet::new();
                                    set.insert(attribute.to_owned());
                                    set
                                });
                        }

                        if name == "Main" {
                            if self.main_type_attrs.is_some() {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "redefined 'Main' type attributes".into(),
                                }));
                            }

                            self.main_type_attrs = Some(tpl_idx);
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

                        if name == "main" {
                            if self.main_fun_attrs.is_some() {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "redefined 'main' function attributes".into(),
                                }));
                            }

                            self.main_fun_attrs = Some(tpl_idx);
                        }
                    }
                }
                FormKind::FunApp => {
                    let name = form.head().to_string();

                    if name != "main" {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected a 'main' function application".into(),
                        }));
                    }

                    if self.main_app.is_some() {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "duplicate 'main' function application".into(),
                        }));
                    }

                    self.main_app = Some(tpl_idx);
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

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct GlobalDefPos {
    pub file: String,
    pub idx: usize,
}

impl GlobalDefPos {
    pub fn new() -> GlobalDefPos {
        GlobalDefPos::default()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct GlobalSymbolTable {
    pub files: BTreeSet<String>,

    // file -> symbol_table
    pub tables: BTreeMap<String, SymbolTable>,
}

impl GlobalSymbolTable {
    pub fn new() -> GlobalSymbolTable {
        GlobalSymbolTable::default()
    }

    pub fn is_defined(&self, name: &str) -> bool {
        for table in self.tables.values() {
            if table.is_defined(name) {
                return true;
            }
        }

        false
    }

    pub fn is_undefined(&self, name: &str) -> bool {
        !self.is_defined(name)
    }

    pub fn find_positions(&self, name: &str) -> Vec<GlobalDefPos> {
        let mut positions = Vec::new();

        for table in self.tables.values() {
            if let Some(idx) = table.find_index(name) {
                let pos = GlobalDefPos {
                    file: table.file.clone(),
                    idx,
                };

                positions.push(pos);
            }
        }

        positions
    }

    pub fn find_values(&self, name: &str) -> Vec<Value> {
        self.find_positions(name)
            .iter()
            .map(|pos| self.tables.get(&pos.file).unwrap().values[pos.idx].clone())
            .collect()
    }

    pub fn add_table(&mut self, table: SymbolTable) -> Result<()> {
        let file = table.file.clone();

        if self.files.contains(&file) {
            if file.is_empty() {
                let mut old_table = self.tables.get(&file).unwrap().clone();
                let values = table.values.clone();

                for value in values.into_iter() {
                    old_table.add_value(value)?;
                }

                self.tables.insert(file.clone(), old_table);
            } else {
                return Err(Error::Semantic(SemanticError {
                    loc: None,
                    desc: format!("file {} already included", file),
                }));
            }
        }

        self.files.insert(file.clone());

        self.tables.insert(file, table);

        Ok(())
    }

    pub fn read_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let table = SymbolTable::from_file(path)?;

        self.add_table(table)
    }

    pub fn read_files<P: AsRef<Path>>(&mut self, paths: &[P]) -> Result<()> {
        for path in paths {
            self.read_file(path)?;
        }

        Ok(())
    }
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

        let res = SymbolTable::from_values(values.clone());

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

        assert!(symbol_table.is_defined("x.D"));
        assert!(symbol_table.is_defined("x.a"));
        assert!(symbol_table.defs.contains_key("x.D"));
        assert!(symbol_table.defs.contains_key("x.a"));

        assert_eq!(symbol_table.defs.get("x.D"), Some(&0));
        assert_eq!(symbol_table.defs.get("x.a"), Some(&0));
        assert_eq!(symbol_table.find_index("x.D"), Some(0));
        assert_eq!(symbol_table.find_index("x.a"), Some(0));
        assert_eq!(symbol_table.find_value("x.D"), Some(values[0].clone()));
        assert_eq!(symbol_table.find_value("x.a"), Some(values[0].clone()));
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

        res = SymbolTable::from_values(values.clone());

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

        s = "(import std.io (prod stdIO))

             (def attrs Main attr1 attr2 attr3)
             (def type Main (Fun IO IO))

             (def sig main Main)
             (def attrs main attrX attrY attrZ)
             (def fun main io io)

             (main stdIO)";

        values = Values::from_str(s).unwrap();

        res = SymbolTable::from_values(values.clone());

        assert!(res.is_ok());

        symbol_table = res.unwrap();

        assert_eq!(symbol_table.file, EMPTY.to_string());

        assert!(symbol_table.is_defined("Main"));
        assert!(symbol_table.is_defined("main"));
        assert!(symbol_table.defs.contains_key("main"));
        assert!(symbol_table.defs.contains_key("Main"));

        assert_eq!(symbol_table.defs.get("Main"), Some(&2));
        assert_eq!(symbol_table.defs.get("main"), Some(&5));
        assert_eq!(symbol_table.find_index("Main"), Some(2));
        assert_eq!(symbol_table.find_index("main"), Some(5));
        assert_eq!(symbol_table.find_value("Main"), Some(values[2].clone()));
        assert_eq!(symbol_table.find_value("main"), Some(values[5].clone()));

        assert_eq!(symbol_table.main_type_attrs, Some(1));
        assert_eq!(symbol_table.main_type, Some(2));
        assert_eq!(symbol_table.main_sig, Some(3));
        assert_eq!(symbol_table.main_fun_attrs, Some(4));
        assert_eq!(symbol_table.main_fun, Some(5));
        assert_eq!(symbol_table.main_app, Some(6));
    }

    #[test]
    fn global_symbol_table() {
        use super::GlobalDefPos;
        use super::GlobalSymbolTable;
        use super::SymbolTable;
        use crate::syntax::EMPTY;
        use crate::values::Values;

        let s = "
            (def type A Prim)
            (def type B Prim)

            (def type Result T E (Sum T E))
            (def type AResult E (Result A E))
            (def type BResult E (Result B E))";

        let values = Values::from_str(s).unwrap();

        let symbol_table = SymbolTable::from_values(values.clone()).unwrap();

        let mut global_symbol_table = GlobalSymbolTable::new();

        let res = global_symbol_table.add_table(symbol_table.clone());

        assert!(res.is_ok());

        assert_eq!(global_symbol_table.files.len(), 1);
        assert!(global_symbol_table.files.contains(EMPTY));
        assert_eq!(global_symbol_table.tables.get(EMPTY), Some(&symbol_table));

        assert!(global_symbol_table.is_defined("A"));
        assert!(global_symbol_table.is_defined("B"));
        assert!(global_symbol_table.is_defined("Result"));
        assert!(global_symbol_table.is_defined("AResult"));
        assert!(global_symbol_table.is_defined("BResult"));

        let mut pos = GlobalDefPos::new();

        pos.idx = 0;
        assert_eq!(global_symbol_table.find_positions("A"), vec![pos.clone()]);
        assert_eq!(
            global_symbol_table.find_values("A"),
            vec![values[0].clone()]
        );

        pos.idx = 1;
        assert_eq!(global_symbol_table.find_positions("B"), vec![pos.clone()]);
        assert_eq!(
            global_symbol_table.find_values("B"),
            vec![values[1].clone()]
        );

        pos.idx = 2;
        assert_eq!(
            global_symbol_table.find_positions("Result"),
            vec![pos.clone()]
        );
        assert_eq!(
            global_symbol_table.find_values("Result"),
            vec![values[2].clone()]
        );

        pos.idx = 3;
        assert_eq!(
            global_symbol_table.find_positions("AResult"),
            vec![pos.clone()]
        );
        assert_eq!(
            global_symbol_table.find_values("AResult"),
            vec![values[3].clone()]
        );

        pos.idx = 4;
        assert_eq!(global_symbol_table.find_positions("BResult"), vec![pos]);
        assert_eq!(
            global_symbol_table.find_values("BResult"),
            vec![values[4].clone()]
        );
    }
}
