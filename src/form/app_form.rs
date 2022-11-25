use crate::error::{Error, SyntacticError};
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
pub enum AppFormVariable {
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

impl Default for AppFormVariable {
    fn default() -> AppFormVariable {
        AppFormVariable::Empty(SimpleValue::new())
    }
}

impl AppFormVariable {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormVariable::Ignore(_) => "_".into(),
            AppFormVariable::Empty(_) => "()".into(),
            AppFormVariable::Panic(_) => "panic".into(),
            AppFormVariable::Prim(prim) => prim.to_string(),
            AppFormVariable::TypeKeyword(keyword) => keyword.to_string(),
            AppFormVariable::TypeSymbol(symbol) => symbol.to_string(),
            AppFormVariable::ValueSymbol(symbol) => symbol.to_string(),
            AppFormVariable::TypePathSymbol(symbol) => symbol.to_string(),
            AppFormVariable::ValuePathSymbol(symbol) => symbol.to_string(),
            AppFormVariable::TypesForm(form) => form.to_string(),
            AppFormVariable::ProdForm(form) => form.to_string(),
            AppFormVariable::FunForm(form) => form.to_string(),
            AppFormVariable::LetForm(form) => form.to_string(),
            AppFormVariable::CaseForm(form) => form.to_string(),
            AppFormVariable::AppForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AppFormVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AppForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub variables: Vec<AppFormVariable>,
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

    pub fn variables_to_string(&self) -> String {
        match self.variables.len() {
            1 => self.variables[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.variables
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "()".to_string(),
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for variable in self.variables.iter() {
            match variable.clone() {
                AppFormVariable::TypesForm(form) => {
                    params.extend(form.all_parameters());
                }
                AppFormVariable::ProdForm(form) => {
                    params.extend(form.all_parameters());
                }
                AppFormVariable::FunForm(form) => {
                    params.extend(form.all_parameters());
                }
                AppFormVariable::LetForm(form) => {
                    params.extend(form.all_parameters());
                }
                AppFormVariable::CaseForm(form) => {
                    params.extend(form.all_parameters());
                }
                AppFormVariable::AppForm(form) => {
                    params.extend(form.all_parameters());
                }
                _ => {}
            }
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];
        vars.push(self.name.clone());

        for variable in self.variables.iter() {
            match variable.clone() {
                AppFormVariable::TypeSymbol(value) => {
                    vars.push(value);
                }
                AppFormVariable::ValueSymbol(value) => {
                    vars.push(value);
                }
                AppFormVariable::TypePathSymbol(value) => {
                    vars.push(value);
                }
                AppFormVariable::ValuePathSymbol(value) => {
                    vars.push(value);
                }
                AppFormVariable::TypesForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormVariable::ProdForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormVariable::FunForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormVariable::LetForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormVariable::CaseForm(form) => {
                    vars.extend(form.all_variables());
                }
                AppFormVariable::AppForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars
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
                    app.variables.push(AppFormVariable::Ignore(value));
                }
                SimpleValue::Empty(_) => {
                    app.variables.push(AppFormVariable::Empty(value));
                }
                SimpleValue::Panic(_) => {
                    app.variables.push(AppFormVariable::Panic(value));
                }
                SimpleValue::Prim(_) => {
                    app.variables.push(AppFormVariable::Prim(value));
                }
                SimpleValue::TypeKeyword(_) => {
                    app.variables.push(AppFormVariable::TypeKeyword(value));
                }
                SimpleValue::ValueSymbol(_) => {
                    app.variables.push(AppFormVariable::ValueSymbol(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    app.variables.push(AppFormVariable::TypeSymbol(value));
                }
                SimpleValue::ValuePathSymbol(_) => {
                    app.variables.push(AppFormVariable::ValuePathSymbol(value));
                }
                SimpleValue::TypePathSymbol(_) => {
                    app.variables.push(AppFormVariable::TypePathSymbol(value));
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
                                app.variables.push(AppFormVariable::Ignore(prim));
                            }
                            ProdFormValue::Panic(prim) => {
                                app.variables.push(AppFormVariable::Panic(prim));
                            }
                            ProdFormValue::Prim(prim) => {
                                app.variables.push(AppFormVariable::Prim(prim));
                            }
                            ProdFormValue::TypeKeyword(keyword) => {
                                app.variables.push(AppFormVariable::TypeKeyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                app.variables.push(AppFormVariable::TypeSymbol(symbol));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                app.variables.push(AppFormVariable::ValueSymbol(symbol));
                            }
                            ProdFormValue::TypePathSymbol(symbol) => {
                                app.variables.push(AppFormVariable::TypePathSymbol(symbol));
                            }
                            ProdFormValue::ValuePathSymbol(symbol) => {
                                app.variables.push(AppFormVariable::ValuePathSymbol(symbol));
                            }
                            ProdFormValue::TypesForm(form) => {
                                app.variables.push(AppFormVariable::TypesForm(form));
                            }
                            ProdFormValue::FunForm(form) => {
                                app.variables.push(AppFormVariable::FunForm(form));
                            }
                            ProdFormValue::LetForm(form) => {
                                app.variables.push(AppFormVariable::LetForm(form));
                            }
                            ProdFormValue::CaseForm(form) => {
                                app.variables.push(AppFormVariable::CaseForm(form));
                            }
                            ProdFormValue::AppForm(form) => {
                                app.variables.push(AppFormVariable::AppForm(form));
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
                    app.variables
                        .push(AppFormVariable::TypesForm(Box::new(form)));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    app.variables.push(AppFormVariable::FunForm(Box::new(form)));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    app.variables.push(AppFormVariable::LetForm(Box::new(form)));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    app.variables
                        .push(AppFormVariable::CaseForm(Box::new(form)));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    app.variables.push(AppFormVariable::AppForm(Box::new(form)));
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
        format!("({} {})", self.name, self.variables_to_string(),)
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
        assert_eq!(form.variables_to_string(), "(prod 0 1 2 3)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap ())";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "unwrap".to_string());
        assert_eq!(form.variables_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(panic E)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "panic".to_string());
        assert_eq!(form.variables_to_string(), "E".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.variables_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap \"io error\")";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.variables_to_string(), "\"io error\"".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.variables_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap (fun (prod a b) (math.+ (prod a b))))";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.variables_to_string(),
            "(fun (prod a b) (math.+ (prod a b)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
