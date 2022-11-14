use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::attrs_form::AttrsForm;
use crate::form::case_form::CaseForm;
use crate::form::def_form::DefForm;
use crate::form::export_form::ExportForm;
use crate::form::form::{Form, FormParam};
use crate::form::fun_form::FunForm;
use crate::form::import_form::ImportForm;
use crate::form::let_form::LetForm;
use crate::form::type_form::TypeForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ProdFormValue {
    Empty,
    Prim(String),
    ValueKeyword(String),
    TypeKeyword(String),
    ValueSymbol(String),
    TypeSymbol(String),
    TypeForm(Box<TypeForm>),
    AttrsForm(Box<AttrsForm>),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    CaseForm(Box<CaseForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    DefForm(Box<DefForm>),
    ImportForm(Box<ImportForm>),
    ExportForm(Box<ExportForm>),
}

impl Default for ProdFormValue {
    fn default() -> ProdFormValue {
        ProdFormValue::Empty
    }
}

impl ProdFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ProdFormValue::Empty => "()".into(),
            ProdFormValue::Prim(prim) => prim.clone(),
            ProdFormValue::ValueKeyword(keyword) => keyword.clone(),
            ProdFormValue::TypeKeyword(keyword) => keyword.clone(),
            ProdFormValue::ValueSymbol(symbol) => symbol.clone(),
            ProdFormValue::TypeSymbol(symbol) => symbol.clone(),
            ProdFormValue::TypeForm(form) => form.to_string(),
            ProdFormValue::AttrsForm(form) => form.to_string(),
            ProdFormValue::ProdForm(form) => form.to_string(),
            ProdFormValue::FunForm(form) => form.to_string(),
            ProdFormValue::CaseForm(form) => form.to_string(),
            ProdFormValue::LetForm(form) => form.to_string(),
            ProdFormValue::AppForm(form) => form.to_string(),
            ProdFormValue::DefForm(form) => form.to_string(),
            ProdFormValue::ImportForm(form) => form.to_string(),
            ProdFormValue::ExportForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ProdFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ProdForm {
    pub tokens: Box<Tokens>,
    pub values: Vec<ProdFormValue>,
}

impl ProdForm {
    pub fn new() -> ProdForm {
        ProdForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values_to_string(&self) -> String {
        self.values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn from_form(form: &Form) -> Result<ProdForm> {
        if form.name != "prod" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a prod keyword".into(),
            }));
        }

        if form.params.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least two values".into(),
            }));
        }

        let mut prod = ProdForm::new();
        prod.tokens = form.tokens.clone();

        for param in form.params.iter() {
            match param.clone() {
                FormParam::Empty => {
                    prod.values.push(ProdFormValue::Empty);
                }
                FormParam::Prim(prim) => {
                    prod.values.push(ProdFormValue::Prim(prim));
                }
                FormParam::ValueKeyword(keyword) => {
                    prod.values.push(ProdFormValue::ValueKeyword(keyword));
                }
                FormParam::TypeKeyword(keyword) => {
                    prod.values.push(ProdFormValue::TypeKeyword(keyword));
                }
                FormParam::ValueSymbol(symbol) => {
                    prod.values.push(ProdFormValue::ValueSymbol(symbol));
                }
                FormParam::TypeSymbol(symbol) => {
                    prod.values.push(ProdFormValue::TypeSymbol(symbol));
                }
                FormParam::Form(form) => {
                    if let Ok(form) = TypeForm::from_form(&form) {
                        prod.values.push(ProdFormValue::TypeForm(Box::new(form)));
                    } else if let Ok(form) = ProdForm::from_form(&form) {
                        prod.values.push(ProdFormValue::ProdForm(Box::new(form)));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        prod.values.push(ProdFormValue::FunForm(Box::new(form)));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        prod.values.push(ProdFormValue::CaseForm(Box::new(form)));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        prod.values.push(ProdFormValue::LetForm(Box::new(form)));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        prod.values.push(ProdFormValue::AppForm(Box::new(form)))
                    } else if let Ok(form) = DefForm::from_form(&form) {
                        prod.values.push(ProdFormValue::DefForm(Box::new(form)))
                    } else if let Ok(form) = ImportForm::from_form(&form) {
                        prod.values.push(ProdFormValue::ImportForm(Box::new(form)))
                    } else if let Ok(form) = ExportForm::from_form(&form) {
                        prod.values.push(ProdFormValue::ExportForm(Box::new(form)))
                    } else if let Ok(form) = AttrsForm::from_form(&form) {
                        prod.values.push(ProdFormValue::AttrsForm(Box::new(form)))
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected ignore".into(),
                    }));
                }
            }
        }

        Ok(prod)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ProdForm> {
        let form = Form::from_tokens(tokens)?;

        ProdForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<ProdForm> {
        let tokens = Tokens::from_str(s)?;

        ProdForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(prod {})", self.values_to_string())
    }
}

impl fmt::Display for ProdForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prod_form_from_str() {
        use super::ProdForm;

        let mut s = "(prod a A)";

        let mut res = ProdForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["a".to_string(), "A".to_string()]
        );
        assert_eq!(form.values_to_string(), "a A".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(prod moduleX.X y)";

        res = ProdForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["moduleX.X".to_string(), "y".to_string()]
        );
        assert_eq!(form.values_to_string(), "moduleX.X y".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(prod 0 (Fun A B))";

        res = ProdForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["0".to_string(), "(Fun A B)".to_string()]
        );
        assert_eq!(form.values_to_string(), "0 (Fun A B)".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
