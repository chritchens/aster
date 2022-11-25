use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::attrs_form::AttrsForm;
use crate::form::case_form::CaseForm;
use crate::form::export_form::ExportForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::import_form::ImportForm;
use crate::form::let_form::LetForm;
use crate::form::sig_form::SigForm;
use crate::form::simple_value::SimpleValue;
use crate::form::type_form::TypeForm;
use crate::form::types_form::TypesForm;
use crate::form::val_form::ValForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ProdFormValue {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Prim(SimpleValue),
    ValueKeyword(SimpleValue),
    TypeKeyword(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    TypesForm(Box<TypesForm>),
    AttrsForm(Box<AttrsForm>),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    CaseForm(Box<CaseForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    TypeForm(Box<TypeForm>),
    SigForm(Box<SigForm>),
    ValForm(Box<ValForm>),
    ImportForm(Box<ImportForm>),
    ExportForm(Box<ExportForm>),
}

impl Default for ProdFormValue {
    fn default() -> ProdFormValue {
        ProdFormValue::Empty(SimpleValue::new())
    }
}

impl ProdFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ProdFormValue::Ignore(_) => "_".into(),
            ProdFormValue::Empty(_) => "()".into(),
            ProdFormValue::Panic(_) => "panic".into(),
            ProdFormValue::Prim(prim) => prim.to_string(),
            ProdFormValue::ValueKeyword(keyword) => keyword.to_string(),
            ProdFormValue::TypeKeyword(keyword) => keyword.to_string(),
            ProdFormValue::ValueSymbol(symbol) => symbol.to_string(),
            ProdFormValue::TypeSymbol(symbol) => symbol.to_string(),
            ProdFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            ProdFormValue::TypePathSymbol(symbol) => symbol.to_string(),
            ProdFormValue::TypesForm(form) => form.to_string(),
            ProdFormValue::AttrsForm(form) => form.to_string(),
            ProdFormValue::ProdForm(form) => form.to_string(),
            ProdFormValue::FunForm(form) => form.to_string(),
            ProdFormValue::CaseForm(form) => form.to_string(),
            ProdFormValue::LetForm(form) => form.to_string(),
            ProdFormValue::AppForm(form) => form.to_string(),
            ProdFormValue::TypeForm(form) => form.to_string(),
            ProdFormValue::SigForm(form) => form.to_string(),
            ProdFormValue::ValForm(form) => form.to_string(),
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

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for value in self.values.iter() {
            match value.clone() {
                ProdFormValue::Ignore(value) => {
                    vars.push(value);
                }
                ProdFormValue::Empty(value) => {
                    vars.push(value);
                }
                ProdFormValue::Panic(value) => {
                    vars.push(value);
                }
                ProdFormValue::Prim(value) => {
                    vars.push(value);
                }
                ProdFormValue::ValueKeyword(value) => {
                    vars.push(value);
                }
                ProdFormValue::TypeKeyword(value) => {
                    vars.push(value);
                }
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
                ProdFormValue::AttrsForm(form) => {
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
                ProdFormValue::TypeForm(_form) => {}
                ProdFormValue::SigForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::ValForm(form) => {
                    vars.extend(form.all_variables());
                }
                ProdFormValue::ImportForm(_form) => {}
                ProdFormValue::ExportForm(_form) => {}
            }
        }

        vars
    }

    pub fn check_linearly_ordered_on_parameters(&self, parameters: &mut Vec<String>) -> Result<()> {
        let bound_variables = self
            .values
            .iter()
            .map(|p| p.to_string())
            .filter(|v| parameters.iter().any(|p| p == v))
            .collect::<Vec<String>>();

        if parameters != &bound_variables {
            if bound_variables.len() != parameters.len() {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-linear use of parameters {}: {}",
                        parameters.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            } else {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-ordered use of parameters {}: {}",
                        parameters.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            }
        }

        parameters.clear();

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<ProdForm> {
        if form.head.to_string() != "prod" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a prod keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least two values".into(),
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
                    SimpleValue::Prim(_) => {
                        prod.values.push(ProdFormValue::Prim(value));
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
                            loc: form.loc(),
                            desc: format!("unxexpected value: {}", x.to_string()),
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
                    } else if let Ok(form) = TypeForm::from_form(&form) {
                        prod.values.push(ProdFormValue::TypeForm(Box::new(form)))
                    } else if let Ok(form) = SigForm::from_form(&form) {
                        prod.values.push(ProdFormValue::SigForm(Box::new(form)))
                    } else if let Ok(form) = ValForm::from_form(&form) {
                        prod.values.push(ProdFormValue::ValForm(Box::new(form)))
                    } else if let Ok(form) = ImportForm::from_form(&form) {
                        prod.values.push(ProdFormValue::ImportForm(Box::new(form)))
                    } else if let Ok(form) = ExportForm::from_form(&form) {
                        prod.values.push(ProdFormValue::ExportForm(Box::new(form)))
                    } else if let Ok(form) = AttrsForm::from_form(&form) {
                        prod.values.push(ProdFormValue::AttrsForm(Box::new(form)))
                    } else {
                        println!("prod_form.rs form: {}", form.to_string());
                        TypeForm::from_form(&form)?;
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
