use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::symbol_name;
use crate::syntax::{is_keyword, is_path_symbol, is_qualified};
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use crate::value::forms::prod_form::{ProdForm, ProdFormValue};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ImportFormDef {
    Empty,
    ValueSymbol(String),
    TypeSymbol(String),
}

impl Default for ImportFormDef {
    fn default() -> ImportFormDef {
        ImportFormDef::Empty
    }
}

impl ImportFormDef {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ImportFormDef::Empty => "()".into(),
            ImportFormDef::ValueSymbol(symbol) => symbol.clone(),
            ImportFormDef::TypeSymbol(symbol) => symbol.clone(),
        }
    }
}

impl fmt::Display for ImportFormDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ImportForm {
    pub tokens: Tokens,
    pub module: String,
    pub qualifier: Option<String>,
    pub defs: Vec<ImportFormDef>,
}

impl ImportForm {
    pub fn new() -> ImportForm {
        ImportForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn defs_to_string(&self) -> String {
        let len = self.defs.len();

        if len > 1 {
            format!(
                "(prod {})",
                self.defs
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else if len == 1 {
            self.defs[0].to_string()
        } else {
            "()".to_string()
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ImportForm> {
        let form = Form::from_tokens(tokens)?;

        if form.name != "import" {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected an import keyword".into(),
            }));
        }

        let mut import = ImportForm::new();
        import.tokens = form.tokens.clone();

        let len = form.params.len();

        if len < 2 {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected at least a module parameter and an empty literal".into(),
            }));
        }

        if len > 3 {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected at most a module, a product of value symbols or a value symbol, and a value symbol"
                    .into(),
            }));
        }

        let module = form.params[0].to_string();

        if is_path_symbol(&module) {
            if is_keyword(&symbol_name(&module)) {
                return Err(Error::Semantic(SemanticError {
                    loc: form.loc(),
                    desc: "a path cannot end with a keyword".into(),
                }));
            }
        } else if is_keyword(&module) {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected a module path".into(),
            }));
        }

        import.module = module;

        if len > 1 {
            match form.params[1].clone() {
                FormParam::Empty => {}
                FormParam::ValueSymbol(symbol) => {
                    if is_qualified(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected an unqualified symbol".into(),
                        }));
                    }

                    if is_keyword(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "unexpected keyword".into(),
                        }));
                    }

                    import.defs.push(ImportFormDef::ValueSymbol(symbol));
                }
                FormParam::TypeSymbol(symbol) => {
                    if is_qualified(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected an unqualified symbol".into(),
                        }));
                    }

                    if is_keyword(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "unexpected keyword".into(),
                        }));
                    }

                    import.defs.push(ImportFormDef::TypeSymbol(symbol));
                }
                FormParam::Form(form) => {
                    let prod = ProdForm::from_form(&form)?;

                    for value in prod.values {
                        match value {
                            ProdFormValue::ValueSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                if is_keyword(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "unexpected keyword".into(),
                                    }));
                                }

                                import.defs.push(ImportFormDef::ValueSymbol(symbol));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                if is_keyword(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "unexpected keyword".into(),
                                    }));
                                }

                                import.defs.push(ImportFormDef::TypeSymbol(symbol));
                            }
                            _ => {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "expected a product of value or type symbols".into(),
                                }));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: "expected a product of symbols or a value symbol".into(),
                    }));
                }
            }
        }

        if len > 2 {
            match form.params[2].clone() {
                FormParam::ValueSymbol(symbol) => {
                    if is_qualified(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected an unqualified symbol".into(),
                        }));
                    }

                    if is_keyword(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "unexpected keyword".into(),
                        }));
                    }

                    import.qualifier = Some(symbol);
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: "expected a value symbol".into(),
                    }));
                }
            }
        }

        Ok(import)
    }

    pub fn from_str(s: &str) -> Result<ImportForm> {
        let tokens = Tokens::from_str(s)?;

        ImportForm::from_tokens(&tokens)
    }

    pub fn to_string(&self) -> String {
        if let Some(qualifier) = self.qualifier.clone() {
            format!(
                "(import {} {} {})",
                self.module,
                self.defs_to_string(),
                qualifier
            )
        } else {
            format!("(import {} {})", self.module, self.defs_to_string(),)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn import_form_from_str() {
        use super::ImportForm;

        let mut s = "(import std.x (prod a B c D) x)";

        let mut res = ImportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.defs_to_string(), "(prod a B c D)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (prod a B c D))";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.defs_to_string(), "(prod a B c D)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x () x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x ())";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x X)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.defs_to_string(), "X".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x x x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
