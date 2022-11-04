use super::{MixedAppForm, MixedAppFormParam};
use super::{SymbolProdForm, SymbolProdFormValue};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_qualified};
use crate::token::Tokens;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ExportForm {
    pub tokens: Tokens,
    pub type_defs: Vec<String>,
    pub value_defs: Vec<String>,
}

impl ExportForm {
    pub fn new() -> ExportForm {
        ExportForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ExportForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        if mixed_app.name != "export" {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected an export keyword".into(),
            }));
        }

        if mixed_app.params.len() != 1 {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a product of exported symbols or an empty literal".into(),
            }));
        }

        let mut export = ExportForm::new();
        export.tokens = mixed_app.tokens.clone();

        for param in mixed_app.params.clone() {
            match param {
                MixedAppFormParam::Empty => {}
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

                                export.value_defs.push(symbol);
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

                                export.type_defs.push(symbol);
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a product of symbols or an empty literal".into(),
                    }));
                }
            }
        }

        Ok(export)
    }

    pub fn from_str(s: &str) -> Result<ExportForm> {
        let tokens = Tokens::from_str(s)?;

        ExportForm::from_tokens(&tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn export_form_from_str() {
        use super::ExportForm;

        let mut s = "(export (prod A))";

        let mut res = ExportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.type_defs, vec!["A".to_string()]);

        s = "(export (prod b C d E))";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.type_defs, vec!["C".to_string(), "E".to_string()]);
        assert_eq!(form.value_defs, vec!["b".to_string(), "d".to_string()]);

        s = "(export ())";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert!(form.type_defs.is_empty());
        assert!(form.value_defs.is_empty());
    }
}
