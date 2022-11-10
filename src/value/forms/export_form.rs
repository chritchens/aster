use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_qualified};
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use crate::value::forms::prod_form::{ProdForm, ProdFormValue};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExportFormDef {
    Empty,
    ValueSymbol(String),
    TypeSymbol(String),
}

impl Default for ExportFormDef {
    fn default() -> ExportFormDef {
        ExportFormDef::Empty
    }
}

impl ExportFormDef {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ExportFormDef::Empty => "()".into(),
            ExportFormDef::ValueSymbol(symbol) => symbol.clone(),
            ExportFormDef::TypeSymbol(symbol) => symbol.clone(),
        }
    }
}

impl fmt::Display for ExportFormDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ExportForm {
    pub tokens: Box<Tokens>,
    pub defs: Vec<ExportFormDef>,
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

    pub fn defs_to_string(&self) -> String {
        match self.defs.len() {
            1 => self.defs[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.defs
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "()".to_string(),
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ExportForm> {
        let form = Form::from_tokens(tokens)?;

        if form.name != "export" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected an export keyword".into(),
            }));
        }

        if form.params.len() != 1 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a product of exported symbols or an empty literal".into(),
            }));
        }

        let mut export = ExportForm::new();
        export.tokens = form.tokens.clone();

        for param in form.params.clone() {
            match param {
                FormParam::Empty => {}
                FormParam::TypeSymbol(symbol) => {
                    if is_qualified(&symbol) {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected an unqualified symbol".into(),
                        }));
                    }

                    if is_keyword(&symbol) {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected keyword".into(),
                        }));
                    }

                    export.defs.push(ExportFormDef::TypeSymbol(symbol));
                }
                FormParam::ValueSymbol(symbol) => {
                    if is_qualified(&symbol) {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected an unqualified symbol".into(),
                        }));
                    }

                    if is_keyword(&symbol) {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected keyword".into(),
                        }));
                    }

                    export.defs.push(ExportFormDef::ValueSymbol(symbol));
                }
                FormParam::Form(form) => {
                    let prod = ProdForm::from_form(&form)?;

                    for value in prod.values {
                        match value {
                            ProdFormValue::ValueSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                if is_keyword(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "unexpected keyword".into(),
                                    }));
                                }

                                export.defs.push(ExportFormDef::ValueSymbol(symbol));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                if is_keyword(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "unexpected keyword".into(),
                                    }));
                                }

                                export.defs.push(ExportFormDef::TypeSymbol(symbol));
                            }
                            _ => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected a product of value or type symbols".into(),
                                }));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
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

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(export {})", self.defs_to_string(),)
    }
}

impl fmt::Display for ExportForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn export_form_from_str() {
        use super::ExportForm;

        let mut s = "(export A)";

        let mut res = ExportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.defs_to_string(), "A".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(export (prod b C d E))";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.defs_to_string(), "(prod b C d E)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(export ())";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());
    }
}
