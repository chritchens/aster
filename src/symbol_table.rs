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

    // maps from a file name to a set of names
    pub imp_paths: BTreeMap<String, BTreeSet<String>>,
    pub exp_defs: BTreeMap<String, BTreeSet<String>>,
    pub def_types: BTreeMap<String, BTreeSet<String>>,
    pub def_prims: BTreeMap<String, BTreeSet<String>>,
    pub def_sums: BTreeMap<String, BTreeSet<String>>,
    pub def_prods: BTreeMap<String, BTreeSet<String>>,
    pub def_sigs: BTreeMap<String, BTreeSet<String>>,
    pub def_funs: BTreeMap<String, BTreeSet<String>>,
    pub def_apps: BTreeMap<String, BTreeSet<String>>,
    pub def_attrs: BTreeMap<String, BTreeSet<String>>,

    pub scoped_def_types: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_prims: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_sums: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_prods: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_sigs: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_funs: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_apps: BTreeMap<String, BTreeSet<String>>,
    pub scoped_def_attrs: BTreeMap<String, BTreeSet<String>>,

    // maps from a value name to a set of value indexes
    pub imports: BTreeMap<String, BTreeSet<usize>>,
    pub exports: BTreeMap<String, BTreeSet<usize>>,
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

    // maps from a value name to a value index
    pub main_type: BTreeMap<String, usize>,
    pub main_sig: BTreeMap<String, usize>,
    pub main_fun: BTreeMap<String, usize>,
    pub main_app: BTreeMap<String, usize>,
    pub main_fun_attrs: BTreeMap<String, usize>,
    pub main_type_attrs: BTreeMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn add_value(&mut self, value_ref: &Value) -> Result<()> {
        let mut value = value_ref.clone();

        self.values.push(value.clone());
        let value_idx = self.values.len() - 1;

        let mut file = "_".into();

        if let Some(f) = value.token.file() {
            self.files.insert(f.clone());
            file = f;
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

                        self.imp_paths
                            .entry(file)
                            .and_modify(|s| {
                                s.insert(name.clone());
                            })
                            .or_insert_with(|| {
                                let mut s = BTreeSet::new();
                                s.insert(name.clone());
                                s
                            });

                        self.imports
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
                    Keyword::Export => {
                        value = value.children[1].clone();

                        if value.children.len() > 1 {
                            let len = value.children.len();

                            for idx in 1..len {
                                let child = value.children[idx].clone();

                                let name = child.name.clone().unwrap();

                                self.exp_defs
                                    .entry(file.clone())
                                    .and_modify(|s| {
                                        s.insert(name.clone());
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(name.clone());
                                        s
                                    });

                                self.exports
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
                            let name = value.name.clone().unwrap();

                            self.exp_defs
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

                            self.exports
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
                    Keyword::Deftype => {
                        let name = value.children[1].name.clone().unwrap();

                        if !value.scope.is_tpl() {
                            if name == "Main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped Main type".into(),
                                }));
                            }

                            self.scoped_def_types
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_types
                                .entry(file.clone())
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                                if self.main_type.get(&file).is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate Main type".into(),
                                    }));
                                }

                                self.main_type.insert(file, value_idx);
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

                            self.scoped_def_sigs
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_sigs
                                .entry(file.clone())
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                                if self.main_sig.get(&file).is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main signature".into(),
                                    }));
                                }

                                self.main_sig.insert(file, value_idx);
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
                            self.scoped_def_prims
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_prims
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                        let name = value.children[1].name.clone().unwrap();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_def_sums
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_sums
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                        let name = value.children[1].name.clone().unwrap();

                        if name == "main" {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "invalid main".into(),
                            }));
                        }

                        if !value.scope.is_tpl() {
                            self.scoped_def_prods
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_prods
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                        let name = value.children[1].name.clone().unwrap();

                        if !value.scope.is_tpl() {
                            if name == "main" {
                                return Err(Error::Semantic(SemanticError {
                                    loc: value.token.loc(),
                                    desc: "invalid scoped main function".into(),
                                }));
                            }

                            self.scoped_def_funs
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_funs
                                .entry(file.clone())
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                                if self.main_fun.get(&file).is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main function".into(),
                                    }));
                                }

                                self.main_fun.insert(file, value_idx);
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

                            self.scoped_def_attrs
                                .entry(file)
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                            self.def_attrs
                                .entry(file.clone())
                                .and_modify(|s| {
                                    s.insert(name.clone());
                                })
                                .or_insert_with(|| {
                                    let mut s = BTreeSet::new();
                                    s.insert(name.clone());
                                    s
                                });

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
                                if self.main_fun_attrs.get(&file).is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main function attributes".into(),
                                    }));
                                }

                                self.main_fun_attrs.insert(file, value_idx);
                            } else if name == "Main" {
                                if self.main_type_attrs.get(&file).is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate Main type attributes".into(),
                                    }));
                                }

                                self.main_type_attrs.insert(file, value_idx);
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

                                        self.scoped_def_types
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_types
                                            .entry(file.clone())
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                            if self.main_type.get(&file).is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate Main type".into(),
                                                }));
                                            }

                                            self.main_type.insert(file, value_idx);
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

                                        self.scoped_def_sigs
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_sigs
                                            .entry(file.clone())
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                            if self.main_sig.get(&file).is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main signature".into(),
                                                }));
                                            }

                                            self.main_sig.insert(file, value_idx);
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
                                        self.scoped_def_prims
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_prims
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.scoped_def_sums
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_sums
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.scoped_def_prods
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_prods
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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

                                        self.scoped_def_funs
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_funs
                                            .entry(file.clone())
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                            if self.main_fun.get(&file).is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main function".into(),
                                                }));
                                            }

                                            self.main_fun.insert(file, value_idx);
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

                                        self.scoped_def_apps
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_apps
                                            .entry(file.clone())
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                            if self.main_app.get(&file).is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main application".into(),
                                                }));
                                            }

                                            self.main_app.insert(file, value_idx);
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

                                        self.scoped_def_attrs
                                            .entry(file)
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                        self.def_attrs
                                            .entry(file.clone())
                                            .and_modify(|s| {
                                                s.insert(name.clone());
                                            })
                                            .or_insert_with(|| {
                                                let mut s = BTreeSet::new();
                                                s.insert(name.clone());
                                                s
                                            });

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
                                            if self.main_fun_attrs.get(&file).is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate main function attributes"
                                                        .into(),
                                                }));
                                            }

                                            self.main_fun_attrs.insert(file, value_idx);
                                        } else if name == "Main" {
                                            if self.main_type_attrs.get(&file).is_some() {
                                                return Err(Error::Semantic(SemanticError {
                                                    loc: name_value.token.loc(),
                                                    desc: "duplicate Main type attributes".into(),
                                                }));
                                            }

                                            self.main_type_attrs.insert(file, value_idx);
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
                                self.scoped_def_prims
                                    .entry(file)
                                    .and_modify(|s| {
                                        s.insert(name.clone());
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(name.clone());
                                        s
                                    });

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
                                self.def_prims
                                    .entry(file)
                                    .and_modify(|s| {
                                        s.insert(name.clone());
                                    })
                                    .or_insert_with(|| {
                                        let mut s = BTreeSet::new();
                                        s.insert(name.clone());
                                        s
                                    });

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
                                if self.main_app.get(&file).is_some() {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: value.token.loc(),
                                        desc: "duplicate main application".into(),
                                    }));
                                }

                                self.main_app.insert(file, value_idx);
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
                        if self.main_app.get(&file).is_some() {
                            return Err(Error::Semantic(SemanticError {
                                loc: value.token.loc(),
                                desc: "duplicate main application".into(),
                            }));
                        }

                        self.main_app.insert(file, value_idx);
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

        assert_eq!(st.imp_paths.get("_").unwrap().len(), 1);
        assert!(st.imp_paths.get("_").unwrap().contains("std.io"));
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

        assert_eq!(st.exp_defs.get("_").unwrap().len(), 1);
        assert!(st.exp_defs.get("_").unwrap().contains(">>"));
        assert_eq!(st.exports.len(), 1);
        assert!(st.exports.contains_key(">>"));

        s = "(export (prod a b c))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exp_defs.get("_").unwrap().len(), 3);
        assert!(st.exp_defs.get("_").unwrap().contains("a"));
        assert!(st.exp_defs.get("_").unwrap().contains("b"));
        assert!(st.exp_defs.get("_").unwrap().contains("c"));
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

        assert_eq!(st.def_types.get("_").unwrap().len(), 1);
        assert!(st.def_types.get("_").unwrap().contains("RGB"));
        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));

        s = "(def RGB (type (Prod UInt UInt UInt)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_types.get("_").unwrap().len(), 1);
        assert!(st.def_types.get("_").unwrap().contains("RGB"));
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

        assert_eq!(st.def_prims.get("_").unwrap().len(), 1);
        assert!(st.def_prims.get("_").unwrap().contains("i"));
        assert_eq!(st.prims.len(), 1);
        assert!(st.prims.contains_key("i"));

        s = "(def i (prim 0))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_prims.get("_").unwrap().len(), 1);
        assert!(st.def_prims.get("_").unwrap().contains("i"));
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

        assert_eq!(st.def_sums.get("_").unwrap().len(), 1);
        assert!(st.def_sums.get("_").unwrap().contains("predicate"));
        assert_eq!(st.sums.len(), 1);
        assert!(st.sums.contains_key("predicate"));

        s = "(def predicate (sum true))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_sums.get("_").unwrap().len(), 1);
        assert!(st.def_sums.get("_").unwrap().contains("predicate"));
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

        assert_eq!(st.def_prods.get("_").unwrap().len(), 1);
        assert!(st.def_prods.get("_").unwrap().contains("result"));
        assert_eq!(st.prods.len(), 1);
        assert!(st.prods.contains_key("result"));

        s = "(def result (prod 1 ()))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_prods.get("_").unwrap().len(), 1);
        assert!(st.def_prods.get("_").unwrap().contains("result"));
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

        assert_eq!(st.def_funs.get("_").unwrap().len(), 1);
        assert!(st.def_funs.get("_").unwrap().contains("rShift"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("rShift"));

        s = "(def rShift (fun x i (>> x i)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_funs.get("_").unwrap().len(), 1);
        assert!(st.def_funs.get("_").unwrap().contains("rShift"));
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
        assert!(st.def_apps.get("_").unwrap().contains("res"));
        assert_eq!(st.apps.len(), 1);
        assert!(st.apps.contains_key("res"));

        s = "(f x y z)";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_apps.len(), 0);
        assert!(!st.def_apps.contains_key("_"));
        assert_eq!(st.apps.len(), 1);
        assert!(st.apps.contains_key("f"));

        s = "(f x y (g z))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.scoped_def_apps.len(), 0);
        assert!(!st.scoped_def_apps.contains_key("_"));
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
        assert!(st.def_attrs.get("_").unwrap().contains("sum"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));

        s = "(def sum (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_attrs.len(), 1);
        assert!(st.def_attrs.get("_").unwrap().contains("sum"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));

        s = "(def Main (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        let value = values[0].clone();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.def_attrs.len(), 1);
        assert!(st.def_attrs.get("_").unwrap().contains("Main"));
        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("Main"));
        assert_eq!(st.main_type_attrs.get("_"), st.position(value).as_ref());
    }

    #[test]
    fn symbol_table_main() {
        use super::SymbolTable;
        use crate::values::Values;

        let mut s = "(defsig main (Fun IO IO))\n(defun main io (id io))";

        let mut values = Values::from_str(s).unwrap();

        let mut st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.get("_").is_some());
        assert_eq!(st.def_sigs.len(), 1);
        assert!(st.def_sigs.get("_").unwrap().contains("main"));
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.def_funs.len(), 1);
        assert!(st.def_funs.get("_").unwrap().contains("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));

        s = "(def main (sig (Fun IO IO)))\n(def main (fun io (id io)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.get("_").is_some());
        assert_eq!(st.def_sigs.len(), 1);
        assert!(st.def_sigs.get("_").unwrap().contains("main"));
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.def_funs.len(), 1);
        assert!(st.def_funs.get("_").unwrap().contains("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));
    }
}
