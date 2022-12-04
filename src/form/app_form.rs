use crate::error::{Error, SyntacticError};
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::pair_form::PairForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum AppFormValue {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    PairForm(Box<PairForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    CaseForm(Box<CaseForm>),
    AppForm(Box<AppForm>),
}

impl Default for AppFormValue {
    fn default() -> AppFormValue {
        AppFormValue::Empty(SimpleValue::new())
    }
}

impl AppFormValue {
    pub fn file(&self) -> String {
        match self {
            AppFormValue::Ignore(ignore) => ignore.file(),
            AppFormValue::Empty(empty) => empty.file(),
            AppFormValue::Panic(panic) => panic.file(),
            AppFormValue::Atomic(atomic) => atomic.file(),
            AppFormValue::ValueSymbol(symbol) => symbol.file(),
            AppFormValue::ValuePathSymbol(symbol) => symbol.file(),
            AppFormValue::PairForm(form) => form.file(),
            AppFormValue::FunForm(form) => form.file(),
            AppFormValue::LetForm(form) => form.file(),
            AppFormValue::CaseForm(form) => form.file(),
            AppFormValue::AppForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            AppFormValue::Ignore(ignore) => ignore.loc(),
            AppFormValue::Empty(empty) => empty.loc(),
            AppFormValue::Panic(panic) => panic.loc(),
            AppFormValue::Atomic(atomic) => atomic.loc(),
            AppFormValue::ValueSymbol(symbol) => symbol.loc(),
            AppFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            AppFormValue::PairForm(form) => form.loc(),
            AppFormValue::FunForm(form) => form.loc(),
            AppFormValue::LetForm(form) => form.loc(),
            AppFormValue::CaseForm(form) => form.loc(),
            AppFormValue::AppForm(form) => form.loc(),
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        match self.clone() {
            AppFormValue::PairForm(form) => {
                params.extend(form.all_parameters());
            }
            AppFormValue::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            AppFormValue::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            AppFormValue::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            AppFormValue::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.clone() {
            AppFormValue::ValueSymbol(value) => {
                vars.push(value);
            }
            AppFormValue::ValuePathSymbol(value) => {
                vars.push(value);
            }
            AppFormValue::PairForm(form) => {
                vars.extend(form.all_variables());
            }
            AppFormValue::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            AppFormValue::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            AppFormValue::CaseForm(form) => {
                vars.extend(form.all_variables());
            }
            AppFormValue::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        vars
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormValue::Ignore(_) => "_".into(),
            AppFormValue::Empty(_) => "()".into(),
            AppFormValue::Panic(_) => "panic".into(),
            AppFormValue::Atomic(atomic) => atomic.to_string(),
            AppFormValue::ValueSymbol(symbol) => symbol.to_string(),
            AppFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            AppFormValue::PairForm(form) => form.to_string(),
            AppFormValue::FunForm(form) => form.to_string(),
            AppFormValue::LetForm(form) => form.to_string(),
            AppFormValue::CaseForm(form) => form.to_string(),
            AppFormValue::AppForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AppFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct AppForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub variables: Vec<AppFormValue>,
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
        self.variables
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for variable in self.variables.iter() {
            params.extend(variable.all_parameters());
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];
        vars.push(self.name.clone());

        for variable in self.variables.iter() {
            vars.extend(variable.all_variables());
        }

        vars
    }

    pub fn parse_variables(&mut self, form: &Form) -> Result<()> {
        if form.tail.is_empty() {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a variable".into(),
            }));
        }

        for var in form.tail[0..].iter() {
            match var.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        self.variables.push(AppFormValue::Ignore(value));
                    }
                    SimpleValue::Empty(_) => {
                        self.variables.push(AppFormValue::Empty(value));
                    }
                    SimpleValue::Panic(_) => {
                        self.variables.push(AppFormValue::Panic(value));
                    }
                    SimpleValue::Atomic(_) => {
                        self.variables.push(AppFormValue::Atomic(value));
                    }
                    SimpleValue::ValueSymbol(_) => {
                        self.variables.push(AppFormValue::ValueSymbol(value));
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        self.variables.push(AppFormValue::ValuePathSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unexpected variable".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = PairForm::from_form(&form) {
                        self.variables.push(AppFormValue::PairForm(Box::new(form)));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        self.variables.push(AppFormValue::FunForm(Box::new(form)));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        self.variables.push(AppFormValue::LetForm(Box::new(form)));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        self.variables.push(AppFormValue::CaseForm(Box::new(form)));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        self.variables.push(AppFormValue::AppForm(Box::new(form)));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<AppForm> {
        if form.tail.is_empty() {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a variable".into(),
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
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: x.loc(),
                    desc: "unexpected value".into(),
                }));
            }
        }

        app.parse_variables(&form)?;

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

        let mut s = "(math.+ 0 1 2 3)";

        let mut res = AppForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "math.+".to_string());
        assert_eq!(form.variables_to_string(), "0 1 2 3".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap ())";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "unwrap".to_string());
        assert_eq!(form.variables_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(panic e)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "panic".to_string());
        assert_eq!(form.variables_to_string(), "e".to_string());
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

        s = "(unwrap (fun a b (math.+ a b)))";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.variables_to_string(),
            "(fun a b (math.+ a b))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
