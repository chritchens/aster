use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::types_form::TypesForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ProdFormValue {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueKeyword(SimpleValue),
    TypeKeyword(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    TypesForm(Box<TypesForm>),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    CaseForm(Box<CaseForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
}

impl Default for ProdFormValue {
    fn default() -> ProdFormValue {
        ProdFormValue::Empty(SimpleValue::new())
    }
}

impl ProdFormValue {
    pub fn file(&self) -> String {
        match self {
            ProdFormValue::Ignore(ignore) => ignore.file(),
            ProdFormValue::Empty(empty) => empty.file(),
            ProdFormValue::Panic(panic) => panic.file(),
            ProdFormValue::Atomic(atomic) => atomic.file(),
            ProdFormValue::ValueKeyword(keyword) => keyword.file(),
            ProdFormValue::TypeKeyword(keyword) => keyword.file(),
            ProdFormValue::ValueSymbol(symbol) => symbol.file(),
            ProdFormValue::TypeSymbol(symbol) => symbol.file(),
            ProdFormValue::ValuePathSymbol(symbol) => symbol.file(),
            ProdFormValue::TypePathSymbol(symbol) => symbol.file(),
            ProdFormValue::TypesForm(form) => form.file(),
            ProdFormValue::ProdForm(form) => form.file(),
            ProdFormValue::FunForm(form) => form.file(),
            ProdFormValue::CaseForm(form) => form.file(),
            ProdFormValue::LetForm(form) => form.file(),
            ProdFormValue::AppForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ProdFormValue::Ignore(ignore) => ignore.loc(),
            ProdFormValue::Empty(empty) => empty.loc(),
            ProdFormValue::Panic(panic) => panic.loc(),
            ProdFormValue::Atomic(atomic) => atomic.loc(),
            ProdFormValue::ValueKeyword(keyword) => keyword.loc(),
            ProdFormValue::TypeKeyword(keyword) => keyword.loc(),
            ProdFormValue::ValueSymbol(symbol) => symbol.loc(),
            ProdFormValue::TypeSymbol(symbol) => symbol.loc(),
            ProdFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            ProdFormValue::TypePathSymbol(symbol) => symbol.loc(),
            ProdFormValue::TypesForm(form) => form.loc(),
            ProdFormValue::ProdForm(form) => form.loc(),
            ProdFormValue::FunForm(form) => form.loc(),
            ProdFormValue::CaseForm(form) => form.loc(),
            ProdFormValue::LetForm(form) => form.loc(),
            ProdFormValue::AppForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ProdFormValue::Ignore(_) => "_".into(),
            ProdFormValue::Empty(_) => "()".into(),
            ProdFormValue::Panic(_) => "panic".into(),
            ProdFormValue::Atomic(atomic) => atomic.to_string(),
            ProdFormValue::ValueKeyword(keyword) => keyword.to_string(),
            ProdFormValue::TypeKeyword(keyword) => keyword.to_string(),
            ProdFormValue::ValueSymbol(symbol) => symbol.to_string(),
            ProdFormValue::TypeSymbol(symbol) => symbol.to_string(),
            ProdFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            ProdFormValue::TypePathSymbol(symbol) => symbol.to_string(),
            ProdFormValue::TypesForm(form) => form.to_string(),
            ProdFormValue::ProdForm(form) => form.to_string(),
            ProdFormValue::FunForm(form) => form.to_string(),
            ProdFormValue::CaseForm(form) => form.to_string(),
            ProdFormValue::LetForm(form) => form.to_string(),
            ProdFormValue::AppForm(form) => form.to_string(),
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

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for value in self.values.iter() {
            match value.clone() {
                ProdFormValue::TypesForm(form) => {
                    params.extend(form.all_parameters());
                }
                ProdFormValue::ProdForm(form) => {
                    params.extend(form.all_parameters());
                }
                ProdFormValue::FunForm(form) => {
                    params.extend(form.all_parameters());
                }
                ProdFormValue::CaseForm(form) => {
                    params.extend(form.all_parameters());
                }
                ProdFormValue::LetForm(form) => {
                    params.extend(form.all_parameters());
                }
                ProdFormValue::AppForm(form) => {
                    params.extend(form.all_parameters());
                }
                _ => {}
            }
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for value in self.values.iter() {
            match value.clone() {
                ProdFormValue::ValueSymbol(value) => {
                    vars.push(value);
                }
                ProdFormValue::TypeSymbol(value) => {
                    vars.push(value);
                }
                ProdFormValue::ValuePathSymbol(value) => {
                    vars.push(value);
                }
                ProdFormValue::TypePathSymbol(value) => {
                    vars.push(value);
                }
                ProdFormValue::TypesForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::ProdForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::FunForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::CaseForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::LetForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::AppForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<ProdForm> {
        if form.head.to_string() != "prod" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a prod keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected two values".into(),
            }));
        }

        let mut prod = ProdForm::new();
        prod.tokens = form.tokens.clone();

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Empty(_) => {
                        prod.values.push(ProdFormValue::Empty(value));
                    }
                    SimpleValue::Atomic(_) => {
                        prod.values.push(ProdFormValue::Atomic(value));
                    }
                    SimpleValue::ValueKeyword(_) => {
                        prod.values.push(ProdFormValue::ValueKeyword(value));
                    }
                    SimpleValue::TypeKeyword(_) => {
                        prod.values.push(ProdFormValue::TypeKeyword(value));
                    }
                    SimpleValue::ValueSymbol(_) => {
                        prod.values.push(ProdFormValue::ValueSymbol(value));
                    }
                    SimpleValue::TypeSymbol(_) => {
                        prod.values.push(ProdFormValue::TypeSymbol(value));
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        prod.values.push(ProdFormValue::ValuePathSymbol(value));
                    }
                    SimpleValue::TypePathSymbol(_) => {
                        prod.values.push(ProdFormValue::TypePathSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unxexpected value".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = TypesForm::from_form(&form) {
                        prod.values.push(ProdFormValue::TypesForm(Box::new(form)));
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
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        Ok(prod)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ProdForm> {
        let form = Form::from_tokens(tokens)?;

        ProdForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
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

impl std::str::FromStr for ProdForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
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
