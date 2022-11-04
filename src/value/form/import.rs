use super::{MixedAppForm, MixedAppFormParam};
use super::{SymbolProdForm, SymbolProdFormValue};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_path_symbol, is_qualified};
use crate::syntax::{symbol_name, symbol_with_qualifier};
use crate::token::Tokens;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ImportForm {
    pub tokens: Tokens,
    pub module: String,
    pub qualifier: Option<String>,
    pub type_defs: Vec<String>,
    pub value_defs: Vec<String>,
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

    pub fn from_tokens(tokens: &Tokens) -> Result<ImportForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        if mixed_app.name != "import" {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected an import keyword".into(),
            }));
        }

        let mut import = ImportForm::new();
        import.tokens = mixed_app.tokens.clone();

        let len = mixed_app.params.len();

        if len < 2 {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected at least a module parameter and an empty literal".into(),
            }));
        }

        if len > 3 {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected at most a module, a product of value symbols and a value symbol"
                    .into(),
            }));
        }

        let module = mixed_app.params[0].to_string();

        if is_path_symbol(&module) {
            if is_keyword(&symbol_name(&module)) {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "a path cannot end with a keyword".into(),
                }));
            }
        } else if is_keyword(&module) {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a module path".into(),
            }));
        }

        import.module = module;

        let mut type_defs = vec![];
        let mut value_defs = vec![];

        if len > 1 {
            match mixed_app.params[1].clone() {
                MixedAppFormParam::Empty => {}
                MixedAppFormParam::ValueSymbol(symbol) => {
                    if len > 2 {
                        return Err(Error::Semantic(SemanticError {
                            loc: mixed_app.loc(),
                            desc: "expected a product of symbols or an empty literal".into(),
                        }));
                    }

                    import.qualifier = Some(symbol);
                }
                MixedAppFormParam::MixedApp(app) => {
                    let prod = SymbolProdForm::from_mixed_app(&app)?;

                    for value in prod.values {
                        match value {
                            SymbolProdFormValue::ValueSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: mixed_app.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                if is_keyword(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: mixed_app.loc(),
                                        desc: "unexpected keyword".into(),
                                    }));
                                }

                                value_defs.push(symbol);
                            }
                            SymbolProdFormValue::TypeSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: mixed_app.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                if is_keyword(&symbol) {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: mixed_app.loc(),
                                        desc: "unexpected keyword".into(),
                                    }));
                                }

                                type_defs.push(symbol);
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a product of symbols or a value symbol".into(),
                    }));
                }
            }
        }

        if len > 2 {
            match mixed_app.params[2].clone() {
                MixedAppFormParam::ValueSymbol(symbol) => {
                    import.qualifier = Some(symbol);
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a value symbol".into(),
                    }));
                }
            }
        }

        if let Some(qualifier) = import.qualifier.clone() {
            type_defs = type_defs
                .iter()
                .map(|d| symbol_with_qualifier(d, &qualifier))
                .collect();

            value_defs = value_defs
                .iter()
                .map(|d| symbol_with_qualifier(d, &qualifier))
                .collect();
        }

        import.type_defs = type_defs;
        import.value_defs = value_defs;

        Ok(import)
    }

    pub fn from_str(s: &str) -> Result<ImportForm> {
        let tokens = Tokens::from_str(s)?;

        ImportForm::from_tokens(&tokens)
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
        assert_eq!(form.type_defs, vec!["x.B".to_string(), "x.D".to_string()]);
        assert_eq!(form.value_defs, vec!["x.a".to_string(), "x.c".to_string()]);

        s = "(import std.x (prod a B c D))";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_defs, vec!["B".to_string(), "D".to_string()]);
        assert_eq!(form.value_defs, vec!["a".to_string(), "c".to_string()]);

        s = "(import std.x () x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert!(form.type_defs.is_empty());
        assert!(form.value_defs.is_empty());

        s = "(import std.x ())";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert!(form.type_defs.is_empty());
        assert!(form.value_defs.is_empty());
    }
}
