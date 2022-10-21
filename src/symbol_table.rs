use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::syntax::{Keyword, WILDCARD};
use crate::typing::Type;
use crate::value::Value;
use crate::value::{path_prefix, path_suffix};
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub file: Option<String>,

    pub values: Values,

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
                        let name = value.children[1].qualified_name();

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

                                let name = child.qualified_name();

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
                            let name = value.qualified_name();

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
            self.imports.contains_key(&qualification)
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

    pub fn is_defined_in_scope(&self, tpl_name: &str, name: &str) -> bool {
        if !self.is_defined(tpl_name) {
            return false;
        }

        !self.values.find_in_scope(tpl_name, name).is_empty()
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

        let s = "(import std.io)";

        let values = Values::from_str(s).unwrap();

        let st = SymbolTable::from_values(&values).unwrap();

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

        assert_eq!(st.exports.len(), 1);
        assert!(st.exports.contains_key(">>"));

        s = "(export (prod a b c))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.exports.len(), 3);
        assert!(st.exports.contains_key("a"));
        assert!(st.exports.contains_key("b"));
        assert!(st.exports.contains_key("c"));
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

        s = "(def RGB (type (Prod UInt UInt UInt)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.types.len(), 1);
        assert!(st.types.contains_key("RGB"));
        assert!(st.is_defined("RGB"));
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

        s = "(def i (prim 0))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.prims.len(), 1);
        assert!(st.prims.contains_key("i"));
        assert!(st.is_defined("i"));
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

        s = "(def predicate (sum true))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.sums.len(), 1);
        assert!(st.sums.contains_key("predicate"));
        assert!(st.is_defined("predicate"));
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

        s = "(def result (prod 1 ()))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.prods.len(), 1);
        assert!(st.prods.contains_key("result"));
        assert!(st.is_defined("result"));
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

        s = "(def rShift (fun x i (>> x i)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("rShift"));
        assert!(st.is_defined("rShift"));
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

        s = "(def sum (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("sum"));
        assert!(st.is_defined("sum"));

        s = "(def Main (attrs attr1 attr2 attr3))";

        values = Values::from_str(s).unwrap();

        let value = values[0].clone();

        st = SymbolTable::from_values(&values).unwrap();

        assert_eq!(st.attrs.len(), 1);
        assert!(st.attrs.contains_key("Main"));
        assert!(st.is_defined("Main"));
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
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));
        assert!(st.is_defined("main"));

        s = "(def main (sig (Fun IO IO)))\n(def main (fun io (id io)))";

        values = Values::from_str(s).unwrap();

        st = SymbolTable::from_values(&values).unwrap();

        assert!(st.main_sig.is_some());
        assert_eq!(st.sigs.len(), 1);
        assert!(st.sigs.contains_key("main"));
        assert_eq!(st.funs.len(), 1);
        assert!(st.funs.contains_key("main"));
        assert!(st.is_defined("main"));
    }
}
