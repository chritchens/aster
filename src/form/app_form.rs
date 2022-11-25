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
pub enum AppFormParameter {
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

impl Default for AppFormParameter {
    fn default() -> AppFormParameter {
        AppFormParameter::Empty(SimpleValue::new())
    }
}

impl AppFormParameter {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormParameter::Ignore(_) => "_".into(),
            AppFormParameter::Empty(_) => "()".into(),
            AppFormParameter::Panic(_) => "panic".into(),
            AppFormParameter::Prim(prim) => prim.to_string(),
            AppFormParameter::TypeKeyword(keyword) => keyword.to_string(),
            AppFormParameter::TypeSymbol(symbol) => symbol.to_string(),
            AppFormParameter::ValueSymbol(symbol) => symbol.to_string(),
            AppFormParameter::TypePathSymbol(symbol) => symbol.to_string(),
            AppFormParameter::ValuePathSymbol(symbol) => symbol.to_string(),
            AppFormParameter::TypesForm(form) => form.to_string(),
            AppFormParameter::ProdForm(form) => form.to_string(),
            AppFormParameter::FunForm(form) => form.to_string(),
            AppFormParameter::LetForm(form) => form.to_string(),
            AppFormParameter::CaseForm(form) => form.to_string(),
            AppFormParameter::AppForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AppFormParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AppForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub parameters: Vec<AppFormParameter>,
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

    pub fn parameters_to_string(&self) -> String {
        match self.parameters.len() {
            1 => self.parameters[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.parameters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "()".to_string(),
        }
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];
        vars.push(self.name.clone());

        for param in self.parameters.iter() {
            match param.clone() {
                AppFormParameter::Ignore(value) => {
                    vars.push(value);
                }
                AppFormParameter::Empty(value) => {
                    vars.push(value);
                }
                AppFormParameter::Panic(value) => {
                    vars.push(value);
                }
                AppFormParameter::Prim(value) => {
                    vars.push(value);
                }
                AppFormParameter::TypeKeyword(value) => {
                    vars.push(value);
                }
                AppFormParameter::TypeSymbol(value) => {
                    vars.push(value);
                }
                AppFormParameter::ValueSymbol(value) => {
                    vars.push(value);
                }
                AppFormParameter::TypePathSymbol(value) => {
                    vars.push(value);
                }
                AppFormParameter::ValuePathSymbol(value) => {
                    vars.push(value);
                }
                AppFormParameter::TypesForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormParameter::ProdForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormParameter::FunForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormParameter::LetForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormParameter::CaseForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormParameter::AppForm(form) => {
                    vars.extend(form.all_variables());
                }
            }
        }

        vars
    }

    pub fn check_linearly_ordered_on_parameters(&self, parameters: &mut Vec<String>) -> Result<()> {
        let mut variables: Vec<String> = vec![self.name.to_string()];
        variables.extend(
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
        );

        let bound_variables = variables
            .iter()
            .filter(|v| parameters.iter().any(|p| v == &p))
            .map(|v| v.to_owned())
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
                    app.parameters.push(AppFormParameter::Ignore(value));
                }
                SimpleValue::Empty(_) => {
                    app.parameters.push(AppFormParameter::Empty(value));
                }
                SimpleValue::Panic(_) => {
                    app.parameters.push(AppFormParameter::Panic(value));
                }
                SimpleValue::Prim(_) => {
                    app.parameters.push(AppFormParameter::Prim(value));
                }
                SimpleValue::TypeKeyword(_) => {
                    app.parameters.push(AppFormParameter::TypeKeyword(value));
                }
                SimpleValue::ValueSymbol(_) => {
                    app.parameters.push(AppFormParameter::ValueSymbol(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    app.parameters.push(AppFormParameter::TypeSymbol(value));
                }
                SimpleValue::ValuePathSymbol(_) => {
                    app.parameters
                        .push(AppFormParameter::ValuePathSymbol(value));
                }
                SimpleValue::TypePathSymbol(_) => {
                    app.parameters.push(AppFormParameter::TypePathSymbol(value));
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
                                app.parameters.push(AppFormParameter::Ignore(prim));
                            }
                            ProdFormValue::Panic(prim) => {
                                app.parameters.push(AppFormParameter::Panic(prim));
                            }
                            ProdFormValue::Prim(prim) => {
                                app.parameters.push(AppFormParameter::Prim(prim));
                            }
                            ProdFormValue::TypeKeyword(keyword) => {
                                app.parameters.push(AppFormParameter::TypeKeyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                app.parameters.push(AppFormParameter::TypeSymbol(symbol));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                app.parameters.push(AppFormParameter::ValueSymbol(symbol));
                            }
                            ProdFormValue::TypePathSymbol(symbol) => {
                                app.parameters
                                    .push(AppFormParameter::TypePathSymbol(symbol));
                            }
                            ProdFormValue::ValuePathSymbol(symbol) => {
                                app.parameters
                                    .push(AppFormParameter::ValuePathSymbol(symbol));
                            }
                            ProdFormValue::TypesForm(form) => {
                                app.parameters.push(AppFormParameter::TypesForm(form));
                            }
                            ProdFormValue::FunForm(form) => {
                                app.parameters.push(AppFormParameter::FunForm(form));
                            }
                            ProdFormValue::LetForm(form) => {
                                app.parameters.push(AppFormParameter::LetForm(form));
                            }
                            ProdFormValue::CaseForm(form) => {
                                app.parameters.push(AppFormParameter::CaseForm(form));
                            }
                            ProdFormValue::AppForm(form) => {
                                app.parameters.push(AppFormParameter::AppForm(form));
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
                    app.parameters
                        .push(AppFormParameter::TypesForm(Box::new(form)));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    app.parameters
                        .push(AppFormParameter::FunForm(Box::new(form)));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    app.parameters
                        .push(AppFormParameter::LetForm(Box::new(form)));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    app.parameters
                        .push(AppFormParameter::CaseForm(Box::new(form)));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    app.parameters
                        .push(AppFormParameter::AppForm(Box::new(form)));
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
        format!("({} {})", self.name, self.parameters_to_string(),)
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
        assert_eq!(form.parameters_to_string(), "(prod 0 1 2 3)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap ())";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "unwrap".to_string());
        assert_eq!(form.parameters_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(panic E)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "panic".to_string());
        assert_eq!(form.parameters_to_string(), "E".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap \"io error\")";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "\"io error\"".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap (fun (prod a b) (math.+ (prod a b))))";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.parameters_to_string(),
            "(fun (prod a b) (math.+ (prod a b)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
