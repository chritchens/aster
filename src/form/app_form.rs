use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::simple_value::SimpleValue;
use crate::form::types_form::TypesForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AppFormParam {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Prim(SimpleValue),
    TypeKeyword(SimpleValue),
    TypeSymbol(SimpleValue),
    ValueSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypesForm(Box<TypesForm>),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    CaseForm(Box<CaseForm>),
    AppForm(Box<AppForm>),
}

impl Default for AppFormParam {
    fn default() -> AppFormParam {
        AppFormParam::Empty(SimpleValue::new())
    }
}

impl AppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormParam::Ignore(_) => "_".into(),
            AppFormParam::Empty(_) => "()".into(),
            AppFormParam::Panic(_) => "panic".into(),
            AppFormParam::Prim(prim) => prim.to_string(),
            AppFormParam::TypeKeyword(keyword) => keyword.to_string(),
            AppFormParam::TypeSymbol(symbol) => symbol.to_string(),
            AppFormParam::ValueSymbol(symbol) => symbol.to_string(),
            AppFormParam::TypePathSymbol(symbol) => symbol.to_string(),
            AppFormParam::ValuePathSymbol(symbol) => symbol.to_string(),
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
    pub name: SimpleValue,
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

    pub fn check_linearly_ordered_on_params(&self, params: &mut Vec<String>) -> Result<()> {
        let mut variables: Vec<String> = vec![self.name.to_string()];
        variables.extend(
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
        );

        let bound_variables = variables
            .iter()
            .filter(|v| params.iter().any(|p| v == &p))
            .map(|v| v.to_owned())
            .collect::<Vec<String>>();

        if params != &bound_variables {
            if bound_variables.len() != params.len() {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-linear use of params {}: {}",
                        params.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            } else {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-ordered use of params {}: {}",
                        params.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            }
        }

        params.clear();

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<AppForm> {
        if form.tail.len() != 1 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "a parameter or product of parameters".into(),
            }));
        }

        let mut app = AppForm::new();
        app.tokens = form.tokens.clone();

        let name = form.head.clone();

        match name {
            SimpleValue::Panic(_) => {
                app.name = name;
            }
            SimpleValue::ValueKeyword(_) => {
                app.name = name;
            }
            SimpleValue::ValueSymbol(_) => {
                app.name = name;
            }
            SimpleValue::ValuePathSymbol(_) => {
                app.name = name;
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a value keyword, a value symbol or a value path symbol".into(),
                }));
            }
        }

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Ignore(_) => {
                    app.params.push(AppFormParam::Ignore(value));
                }
                SimpleValue::Empty(_) => {
                    app.params.push(AppFormParam::Empty(value));
                }
                SimpleValue::Panic(_) => {
                    app.params.push(AppFormParam::Panic(value));
                }
                SimpleValue::Prim(_) => {
                    app.params.push(AppFormParam::Prim(value));
                }
                SimpleValue::TypeKeyword(_) => {
                    app.params.push(AppFormParam::TypeKeyword(value));
                }
                SimpleValue::ValueSymbol(_) => {
                    app.params.push(AppFormParam::ValueSymbol(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    app.params.push(AppFormParam::TypeSymbol(value));
                }
                SimpleValue::ValuePathSymbol(_) => {
                    app.params.push(AppFormParam::ValuePathSymbol(value));
                }
                SimpleValue::TypePathSymbol(_) => {
                    app.params.push(AppFormParam::TypePathSymbol(value));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected parameter: {}", x.to_string()),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::Ignore(prim) => {
                                app.params.push(AppFormParam::Ignore(prim));
                            }
                            ProdFormValue::Panic(prim) => {
                                app.params.push(AppFormParam::Panic(prim));
                            }
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
                            ProdFormValue::TypePathSymbol(symbol) => {
                                app.params.push(AppFormParam::TypePathSymbol(symbol));
                            }
                            ProdFormValue::ValuePathSymbol(symbol) => {
                                app.params.push(AppFormParam::ValuePathSymbol(symbol));
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
        }

        Ok(app)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AppForm> {
        let form = Form::from_tokens(tokens)?;

        AppForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
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

impl std::str::FromStr for AppForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn app_form_from_str() {
        use super::AppForm;

        let mut s = "(math.+ (prod 0 1 2 3))";

        let mut res = AppForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "math.+".to_string());
        assert_eq!(form.params_to_string(), "(prod 0 1 2 3)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap ())";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "unwrap".to_string());
        assert_eq!(form.params_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(panic E)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "panic".to_string());
        assert_eq!(form.params_to_string(), "E".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap \"io error\")";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

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
