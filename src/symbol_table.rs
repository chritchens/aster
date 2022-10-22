use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::syntax::{is_type_symbol, Keyword, WILDCARD};
use crate::typing::Type;
use crate::value::Value;
use crate::value::{add_prefix, path_prefix, path_suffix};
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub file: Option<String>,

    pub values: Values,

    pub imported_modules: BTreeMap<String, BTreeSet<usize>>,
    pub imported_type_symbols: BTreeMap<String, BTreeSet<usize>>,
    pub imported_value_symbols: BTreeMap<String, BTreeSet<usize>>,
    pub exported_type_symbols: BTreeMap<String, BTreeSet<usize>>,
    pub exported_value_symbols: BTreeMap<String, BTreeSet<usize>>,

    pub types: BTreeMap<String, BTreeSet<usize>>,
    pub prims: BTreeMap<String, BTreeSet<usize>>,
    pub sums: BTreeMap<String, BTreeSet<usize>>,
    pub prods: BTreeMap<String, BTreeSet<usize>>,
    pub sigs: BTreeMap<String, BTreeSet<usize>>,
    pub funs: BTreeMap<String, BTreeSet<usize>>,
    pub apps: BTreeMap<String, BTreeSet<usize>>,
    pub attrs: BTreeMap<String, BTreeSet<usize>>,

    pub scoped_types: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_prims: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_sums: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_prods: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_sigs: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_funs: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_apps: BTreeMap<String, BTreeSet<usize>>,
    pub scoped_attrs: BTreeMap<String, BTreeSet<usize>>,

    pub main_type: Option<usize>,
    pub main_sig: Option<usize>,
    pub main_fun: Option<usize>,
    pub main_app: Option<usize>,
    pub main_fun_attrs: Option<usize>,
    pub main_type_attrs: Option<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn add_value(&mut self, value_ref: &Value) -> Result<()> {
        let mut value = value_ref.clone();

        self.values.push(value.clone());
        let value_idx = self.values.len() - 1;

        let file = value.token.file();
        if self.file.is_none() {
            self.file = Some(file);
        }

        if let Some(Type::App(types)) = value.typing.clone() {
            if types[0] == Type::Builtin {
                let name = value.qualified_name();
                let keyword = Keyword::from_str(&name)?;

                match keyword {
                    Keyword::Import => {
                        let module_name = value.children[1].qualified_name();

                        self.imported_modules
                            .entry(module_name)
                            .and_modify(|s| {
                                s.insert(value_idx);
                            })
                            .or_insert_with(|| {
                                let mut s = BTreeSet::new();
                                s.insert(value_idx);
                                s
                            });

                        if value.children.len() > 2 {
                            let imports_value = value.children[2].clone();

                            let qualification = if value.children.len() == 4 {
                                value.children[3].name()
                            } else {
                                "".into()
                            };

                            if imports_value.children.len() > 1 {
                                let len = imports_value.children.len();

                                for idx in 1..len {
                                    let child = imports_value.children[idx].clone();

                                    let name = child.name();
                                    let qualified_name = add_prefix(&name, &qualification);

                                    if is_type_symbol(&name) {
                                        self.imported_type_symbols
                                            .entry(qualified_name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.imported_value_symbols
                                            .entry(qualified_name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    }
                                }
                            } else if !qualification.is_empty() {
                                let name = WILDCARD.to_string();
                                let qualified_name = add_prefix(&name, &qualification);

                                self.imported_type_symbols
                                    .entry(qualified_name.clone())
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });

                                self.imported_value_symbols
                                    .entry(qualified_name)
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
                            }
                        }
                    }
                    Keyword::Export => {
                        value = value.children[1].clone();

                        if value.children.len() > 1 {
                            let len = value.children.len();

                            for idx in 1..len {
                                let child = value.children[idx].clone();

                                let name = child.qualified_name();

                                if is_type_symbol(&name) {
                                    self.exported_type_symbols
                                        .entry(name)
                                        .and_modify(|s| {
                                            s.insert(value_idx);
                                        })
                                        .or_insert_with(|| {
                                            let mut s = BTreeSet::new();
                                            s.insert(value_idx);
                                            s
                                        });
                                } else {
                                    self.exported_value_symbols
                                        .entry(name)
                                        .and_modify(|s| {
                                            s.insert(value_idx);
                                        })
                                        .or_insert_with(|| {
                                            let mut s = BTreeSet::new();
                                            s.insert(value_idx);
                                            s
                                        });
                                }
                            }
                        } else {
                            let name = value.qualified_name();

                            if is_type_symbol(&name) {
                                self.exported_type_symbols
                                    .entry(name)
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
                            } else {
                                self.exported_value_symbols
                                    .entry(name)
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
                            }
                        }
                    }
                    Keyword::Deftype => {
                        let name = value.children[1].qualified_name();

                        if !value.scope.is_tpl() {
                            if name == "Main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped Main type".into(),
                                }));
                            }

                            self.scoped_types
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.types
                                .entry(name.clone())
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });

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
                        let name = value.children[1].qualified_name();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main signature".into(),
                                }));
                            }

                            self.scoped_sigs
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.sigs
                                .entry(name.clone())
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });

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
                        let name = value.children[1].qualified_name();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_prims
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.prims
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        }
                    }
                    Keyword::Defsum => {
                        let name = value.children[1].qualified_name();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_sums
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.sums
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        }
                    }
                    Keyword::Defprod => {
                        let name = value.children[1].qualified_name();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_prods
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.prods
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        }
                    }
                    Keyword::Defun => {
                        let name = value.children[1].qualified_name();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main function".into(),
                                }));
                            }

                            self.scoped_funs
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.funs
                                .entry(name.clone())
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });

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
                        let name = value.children[1].qualified_name();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main attributes".into(),
                                }));
                            }

                            self.scoped_attrs
                                .entry(name)
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.attrs
                                .entry(name.clone())
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });

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

                        let name = value.children[1].qualified_name();
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

                                        self.scoped_types
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.types
                                            .entry(name.clone())
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });

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

                                        self.scoped_sigs
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.sigs
                                            .entry(name.clone())
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });

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
                                        self.scoped_prims
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.prims
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
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
                                        self.scoped_sums
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.sums
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
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
                                        self.scoped_prods
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.prods
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
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

                                        self.scoped_funs
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.funs
                                            .entry(name.clone())
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });

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

                                        self.scoped_apps
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.apps
                                            .entry(name.clone())
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });

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

                                        self.scoped_attrs
                                            .entry(name)
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });
                                    } else {
                                        self.attrs
                                            .entry(name.clone())
                                            .and_modify(|s| {
                                                s.insert(value_idx);
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(value_idx);
                                                s
                                            });

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
                            let name = value.qualified_name();

                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid main".into(),
                                }));
                            }

                            if !value.scope.is_tpl() {
                                self.scoped_prims
                                    .entry(name)
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
                            } else {
                                self.prims
                                    .entry(name)
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
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
                            value.children[1].qualified_name()
                        } else {
                            WILDCARD.to_string()
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
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
                            } else {
                                self.prims
                                    .entry(name)
                                    .and_modify(|s| {
                                        s.insert(value_idx);
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(value_idx);
                                        s
                                    });
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
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });
                        } else {
                            self.apps
                                .entry(name.clone())
                                .and_modify(|s| {
                                    s.insert(value_idx);
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(value_idx);
                                    s
                                });

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
                let name = value.children[0].qualified_name();

                if !value.scope.is_tpl() {
                    if name == "main" {
                        return Err(Error::Semantic(SemanticError {
                            loc: value.token.loc(),
                            desc: "invalid scoped main application".into(),
                        }));
                    }

                    self.scoped_apps
                        .entry(name)
                        .and_modify(|s| {
                            s.insert(value_idx);
                        })
                        .or_insert_with(|| {
                            let mut s = BTreeSet::new();
                            s.insert(value_idx);
                            s
                        });
                } else {
                    self.apps
                        .entry(name.clone())
                        .and_modify(|s| {
                            s.insert(value_idx);
                        })
                        .or_insert_with(|| {
                            let mut s = BTreeSet::new();
                            s.insert(value_idx);
                            s
                        });

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
        let file = values[0].file();

        for value in values.clone().into_iter() {
            if value.file() != file {
                return Err(Error::Semantic(SemanticError {
                    loc: value.token.loc(),
                    desc: "invalid value file".into(),
                }));
            }

            st.add_value(&value)?;
        }

        Ok(st)
    }

    pub fn position(&self, value: Value) -> Option<usize> {
        self.values.position(value)
    }

    pub fn is_defined(&self, qualified_name: &str) -> bool {
        let unqualified_name = path_suffix(qualified_name);

        if unqualified_name != qualified_name {
            let qualification = path_prefix(qualified_name);
            self.imported_modules.contains_key(&qualification)
        } else if unqualified_name == "main" {
            self.main_sig.is_some()
                || self.main_fun.is_some()
                || self.main_fun_attrs.is_some()
                || self.main_app.is_some()
        } else if unqualified_name == "Main" {
            self.main_type.is_some() || self.main_type_attrs.is_some()
        } else {
            self.types.contains_key(&unqualified_name)
                || self.prims.contains_key(&unqualified_name)
                || self.sums.contains_key(&unqualified_name)
                || self.prods.contains_key(&unqualified_name)
                || self.sigs.contains_key(&unqualified_name)
                || self.funs.contains_key(&unqualified_name)
                || self.apps.contains_key(&unqualified_name)
                || self.attrs.contains_key(&unqualified_name)
        }
    }

    pub fn is_defined_in_scope(
        &self,
        tpl_name: &str,
        tpl_def: Option<&str>,
        tpl_path: &[usize],
        name: &str,
    ) -> bool {
        let defined_tpl = tpl_def.unwrap_or(tpl_name);

        if !self.is_defined(defined_tpl) {
            return false;
        }

        !self
            .values
            .find_in_scope(tpl_name, tpl_path, name)
            .is_empty()
    }

    pub fn is_well_defined(&self, tpl_name: &str, qualified_name: Option<&str>) -> bool {
        if let Some(defined) = qualified_name {
            if !self.is_defined(defined) {
                false
            } else {
                !self
                    .values
                    .find_qualified_with_keyword(tpl_name, defined)
                    .is_empty()
            }
        } else {
            if !self.is_defined(tpl_name) {
                return false;
            }

            self.apps.contains_key(tpl_name)
        }
    }

    pub fn is_well_defined_in_scope(
        &self,
        tpl_name: &str,
        tpl_def: Option<&str>,
        tpl_path: &[usize],
        keyword: Option<&str>,
        name: &str,
    ) -> bool {
        if !self.is_well_defined(tpl_name, tpl_def) {
            return false;
        }

        !self
            .values
            .find_in_scope_with_keyword(tpl_name, tpl_path, keyword, name)
            .is_empty()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct GlobalSymbolTable {
    pub files: BTreeSet<String>,

    pub symbol_tables: BTreeMap<String, SymbolTable>,
}

impl GlobalSymbolTable {
    pub fn new() -> GlobalSymbolTable {
        GlobalSymbolTable::default()
    }

    pub fn add_value(&mut self, value: Value) -> Result<()> {
        let file = value.file();

        if !self.files.contains(&file) {
            self.files.insert(file.clone());

            let mut st = SymbolTable::new();
            st.add_value(&value)?;

            self.symbol_tables.insert(file, st);
        } else {
            let mut st = self.symbol_tables.get(&file).unwrap().to_owned();
            st.add_value(&value)?;
            self.symbol_tables.insert(file, st);
        }

        Ok(())
    }

    pub fn add_values(&mut self, values: Values) -> Result<()> {
        for value in values {
            self.add_value(value)?;
        }

        Ok(())
    }

    pub fn from_values(values: Values) -> Result<GlobalSymbolTable> {
        let mut gst = GlobalSymbolTable::new();

        gst.add_values(values)?;

        Ok(gst)
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

        let mut s = "(import std.io)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.imported_modules.len(), 1);
        assert!(st.imported_modules.contains_key("std.io"));

        s = "(import std.io (prod println IO))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.imported_modules.len(), 1);
        assert!(st.imported_modules.contains_key("std.io"));
        assert!(st.imported_value_symbols.contains_key("println"));
        assert!(st.imported_type_symbols.contains_key("IO"));

        s = "(import std.io (prod println IO) io)";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.imported_modules.len(), 1);
        assert!(st.imported_modules.contains_key("std.io"));
        assert!(st.imported_value_symbols.contains_key("io.println"));
        assert!(st.imported_type_symbols.contains_key("io.IO"));

        s = "(import std.io ())";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.imported_modules.len(), 1);
        assert!(st.imported_modules.contains_key("std.io"));
        assert_eq!(st.imported_value_symbols.len(), 0);
        assert_eq!(st.imported_type_symbols.len(), 0);

        s = "(import std.io () io)";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.imported_modules.len(), 1);
        assert!(st.imported_modules.contains_key("std.io"));
        assert_eq!(st.imported_value_symbols.len(), 1);
        assert!(st.imported_value_symbols.contains_key("io._"));
        assert_eq!(st.imported_type_symbols.len(), 1);
        assert!(st.imported_type_symbols.contains_key("io._"));
    }

    #[test]
    fn symbol_table_exports() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(export >>)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exported_value_symbols.len(), 1);
        assert!(st.exported_value_symbols.contains_key(">>"));

        s = "(export (prod a b c))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exported_value_symbols.len(), 3);
        assert!(st.exported_value_symbols.contains_key("a"));
        assert!(st.exported_value_symbols.contains_key("b"));
        assert!(st.exported_value_symbols.contains_key("c"));

        s = "(export (prod A B C))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exported_type_symbols.len(), 3);
        assert!(st.exported_type_symbols.contains_key("A"));
        assert!(st.exported_type_symbols.contains_key("B"));
        assert!(st.exported_type_symbols.contains_key("C"));
    }

    #[test]
    fn symbol_table_types() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(deftype RGB (Prod UInt UInt UInt))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));
        assert!(st.is_defined("RGB"));
        assert!(st.is_well_defined("deftype", Some("RGB")));

        s = "(def RGB (type (Prod UInt UInt UInt)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));
        assert!(st.is_defined("RGB"));
        assert!(st.is_well_defined("def", Some("RGB")));
    }

    #[test]
    fn symbol_table_prims() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defprim i 0)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.prims.len(), 1);
        assert!(st.prims.contains_key("i"));
        assert!(st.is_defined("i"));
        assert!(st.is_well_defined("defprim", Some("i")));

        s = "(def i (prim 0))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.prims.len(), 1);
        assert!(st.prims.contains_key("i"));
        assert!(st.is_defined("i"));
        assert!(st.is_well_defined("def", Some("i")));
    }

    #[test]
    fn symbol_table_sums() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defsum predicate true)";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.sums.len(), 1);
        assert!(st.sums.contains_key("predicate"));
        assert!(st.is_defined("predicate"));
        assert!(st.is_well_defined("defsum", Some("predicate")));

        s = "(def predicate (sum true))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.sums.len(), 1);
        assert!(st.sums.contains_key("predicate"));
        assert!(st.is_defined("predicate"));
        assert!(st.is_well_defined("def", Some("predicate")));
    }

    #[test]
    fn symbol_table_prods() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defprod result 1 ())";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.prods.len(), 1);
        assert!(st.prods.contains_key("result"));
        assert!(st.is_defined("result"));
        assert!(st.is_well_defined("defprod", Some("result")));

        s = "(def result (prod 1 ()))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.prods.len(), 1);
        assert!(st.prods.contains_key("result"));
        assert!(st.is_defined("result"));
        assert!(st.is_well_defined("def", Some("result")));
    }

    #[test]
    fn symbol_table_funs() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defun rShift x i (>> x i))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("rShift"));
        assert!(st.is_defined("rShift"));
        assert!(st.is_well_defined("defun", Some("rShift")));

        s = "(def rShift (fun x i (>> x i)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("rShift"));
        assert!(st.is_defined("rShift"));
        assert!(st.is_well_defined("def", Some("rShift")));
    }

    #[test]
    fn symbol_table_apps() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(def res (app f x y z))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.apps.len(), 1);
        assert!(st.apps.contains_key("res"));
        assert!(st.is_defined("res"));
        assert!(st.is_well_defined("def", Some("res")));

        s = "(f x y z)";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.apps.len(), 1);
        assert!(st.apps.contains_key("f"));

        s = "(f x y (g z))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

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

        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));
        assert!(st.is_well_defined("defattrs", Some("sum")));

        s = "(def sum (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));
        assert!(st.is_defined("sum"));
        assert!(st.is_well_defined("def", Some("sum")));

        s = "(def Main (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        let value = values[0].clone();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("Main"));
        assert!(st.is_defined("Main"));
        assert_eq!(st.main_type_attrs, st.position(value));
        assert!(st.is_well_defined("def", Some("Main")));
    }

    #[test]
    fn symbol_table_main() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defsig main (Fun IO IO))\n(defun main io (id io))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.is_some());
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));
        assert!(st.is_defined("main"));
        assert!(st.is_well_defined("defsig", Some("main")));

        s = "(def main (sig (Fun IO IO)))\n(def main (fun io (id io)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.is_some());
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));
        assert!(st.is_defined("main"));
        assert!(st.is_well_defined("def", Some("main")));
    }

    #[test]
    fn symbol_table_scoping() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(defun f x y (let (defun g (+ x y)) (g x y)))";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

        assert!(st.funs.contains_key("f"));
        assert!(st.scoped_funs.contains_key("g"));
        assert!(st.scoped_apps.contains_key("+"));
        assert!(st.scoped_apps.contains_key("g"));
        assert!(st.funs.get("f") < st.scoped_apps.get("+"));
        assert!(st.scoped_funs.get("g") < st.scoped_apps.get("g"));
        assert!(st.is_defined("f"));
        assert!(st.is_well_defined("defun", Some("f")));
        assert!(st.is_defined_in_scope("defun", Some("f"), &[0], "g"));
        assert!(st.is_well_defined_in_scope("defun", Some("f"), &[0], Some("defun"), "g"));
    }
}
