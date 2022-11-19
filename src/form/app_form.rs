use crate::error::{Error, SyntacticError};
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormParam};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::types_form::TypesForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_value_keyword, is_value_symbol, symbol_name};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AppFormParam {
    Empty,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    TypesForm(Box<TypesForm>),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    CaseForm(Box<CaseForm>),
    AppForm(Box<AppForm>),
}

impl Default for AppFormParam {
    fn default() -> AppFormParam {
        AppFormParam::Empty
    }
}

impl AppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormParam::Empty => "()".into(),
            AppFormParam::Prim(prim) => prim.clone(),
            AppFormParam::TypeKeyword(keyword) => keyword.clone(),
            AppFormParam::TypeSymbol(symbol) => symbol.clone(),
            AppFormParam::ValueSymbol(symbol) => symbol.clone(),
            AppFormParam::TypesForm(form) => form.to_string(),
            AppFormParam::ProdForm(form) => form.to_string(),
            AppFormParam::FunForm(form) => form.to_string(),
            AppFormParam::LetForm(form) => form.to_string(),
            AppFormParam::CaseForm(form) => form.to_string(),
            AppFormParam::AppForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AppFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AppForm {
    pub tokens: Box<Tokens>,
    pub name: String,
    pub params: Vec<AppFormParam>,
}

impl AppForm {
    pub fn new() -> AppForm {
        AppForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn params_to_string(&self) -> String {
        match self.params.len() {
            1 => self.params[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "()".to_string(),
        }
    }

    pub fn from_form(form: &Form) -> Result<AppForm> {
        if !is_value_symbol(&symbol_name(&form.name)) && !is_value_keyword(&form.name) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a value symbol".into(),
            }));
        }

        if form.params.len() != 1 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "a parameter or product of parameters".into(),
            }));
        }

        let mut app = AppForm::new();
        app.tokens = form.tokens.clone();
        app.name = form.name.clone();

        match form.params[0].clone() {
            FormParam::Empty => {}
            FormParam::Prim(prim) => {
                app.params.push(AppFormParam::Prim(prim));
            }
            FormParam::TypeKeyword(keyword) => {
                app.params.push(AppFormParam::TypeKeyword(keyword));
            }
            FormParam::ValueSymbol(symbol) => {
                app.params.push(AppFormParam::ValueSymbol(symbol));
            }
            FormParam::TypeSymbol(symbol) => {
                app.params.push(AppFormParam::TypeSymbol(symbol));
            }
            FormParam::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::Prim(prim) => {
                                app.params.push(AppFormParam::Prim(prim));
                            }
                            ProdFormValue::TypeKeyword(keyword) => {
                                app.params.push(AppFormParam::TypeKeyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                app.params.push(AppFormParam::TypeSymbol(symbol));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                app.params.push(AppFormParam::ValueSymbol(symbol));
                            }
                            ProdFormValue::TypesForm(form) => {
                                app.params.push(AppFormParam::TypesForm(form));
                            }
                            ProdFormValue::FunForm(form) => {
                                app.params.push(AppFormParam::FunForm(form));
                            }
                            ProdFormValue::LetForm(form) => {
                                app.params.push(AppFormParam::LetForm(form));
                            }
                            ProdFormValue::CaseForm(form) => {
                                app.params.push(AppFormParam::CaseForm(form));
                            }
                            ProdFormValue::AppForm(form) => {
                                app.params.push(AppFormParam::AppForm(form));
                            }
                            _ => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected a product of keywords or symbols".into(),
                                }));
                            }
                        }
                    }
                } else if let Ok(form) = TypesForm::from_form(&form) {
                    app.params.push(AppFormParam::TypesForm(Box::new(form)));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    app.params.push(AppFormParam::FunForm(Box::new(form)));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    app.params.push(AppFormParam::LetForm(Box::new(form)));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    app.params.push(AppFormParam::CaseForm(Box::new(form)));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    app.params.push(AppFormParam::AppForm(Box::new(form)));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a product of keywords or symbols".into(),
                    }));
                }
            }
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: format!("unexpected parameter: {}", x.to_string()),
                }));
            }
        }

        Ok(app)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AppForm> {
        let form = Form::from_tokens(tokens)?;

        AppForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<AppForm> {
        let tokens = Tokens::from_str(s)?;

        AppForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("({} {})", self.name, self.params_to_string(),)
    }
}

impl fmt::Display for AppForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn app_form_from_str() {
        use super::AppForm;
        use super::AppFormParam;

        let mut s = "(math.+ (prod 0 1 2 3))";

        let mut res = AppForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "math.+".to_string());
        assert_eq!(form.params_to_string(), "(prod 0 1 2 3)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap ())";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "unwrap".to_string());
        assert!(form.params.is_empty());
        assert_eq!(form.params_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(panic E)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "panic".to_string());
        assert_eq!(form.params, vec![AppFormParam::TypeSymbol("E".into())]);
        assert_eq!(form.params_to_string(), "E".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params, vec![AppFormParam::ValueSymbol("stdIO".into())]);
        assert_eq!(form.params_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap \"io error\")";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params, vec![AppFormParam::Prim("\"io error\"".into())]);
        assert_eq!(form.params_to_string(), "\"io error\"".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap (fun (prod a b) (math.+ (prod a b))))";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.params_to_string(),
            "(fun (prod a b) (math.+ (prod a b)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
