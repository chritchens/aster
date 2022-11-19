use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormParam};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::ProdForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_qualified, is_value_symbol};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValFormValue {
    Empty,
    Prim(String),
    ValueSymbol(String),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    CaseForm(Box<CaseForm>),
}

impl Default for ValFormValue {
    fn default() -> ValFormValue {
        ValFormValue::Empty
    }
}

impl ValFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ValFormValue::Empty => "()".into(),
            ValFormValue::Prim(prim) => prim.clone(),
            ValFormValue::ValueSymbol(symbol) => symbol.clone(),
            ValFormValue::ProdForm(form) => form.to_string(),
            ValFormValue::FunForm(form) => form.to_string(),
            ValFormValue::LetForm(form) => form.to_string(),
            ValFormValue::AppForm(form) => form.to_string(),
            ValFormValue::CaseForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ValFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ValForm {
    pub tokens: Box<Tokens>,
    pub name: String,
    pub value: ValFormValue,
}

impl ValForm {
    pub fn new() -> ValForm {
        ValForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn is_empty_literal(&self) -> bool {
        match self.value {
            ValFormValue::Empty => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self.value {
            ValFormValue::Prim(_) => true,
            _ => false,
        }
    }

    pub fn is_value_symbol(&self) -> bool {
        match self.value {
            ValFormValue::ValueSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_product_form(&self) -> bool {
        match self.value {
            ValFormValue::ProdForm(_) => true,
            _ => false,
        }
    }

    pub fn is_function_form(&self) -> bool {
        match self.value {
            ValFormValue::FunForm(_) => true,
            _ => false,
        }
    }

    pub fn is_application_form(&self) -> bool {
        match self.value {
            ValFormValue::AppForm(_) => true,
            _ => false,
        }
    }

    pub fn is_let_form(&self) -> bool {
        match self.value {
            ValFormValue::LetForm(_) => true,
            _ => false,
        }
    }

    pub fn is_case_form(&self) -> bool {
        match self.value {
            ValFormValue::CaseForm(_) => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        self.is_empty_literal()
            || self.is_primitive()
            || self.is_value_symbol()
            || self.is_product_form()
            || self.is_function_form()
            || self.is_case_form()
            || (self.is_let_form() && is_value_symbol(&self.name))
            || (self.is_application_form() && is_value_symbol(&self.name))
    }

    pub fn from_form(form: &Form) -> Result<ValForm> {
        if form.name != "val" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a val keyword".into(),
            }));
        }

        if form.params.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and an empty literal or a primitive or a symbol or a form"
                    .into(),
            }));
        }

        let mut val = ValForm::new();
        val.tokens = form.tokens.clone();

        match form.params[0].clone() {
            FormParam::ValueSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                val.name = symbol;
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a value symbol".into(),
                }));
            }
        }

        match form.params[1].clone() {
            FormParam::Empty => {}
            FormParam::Prim(prim) => {
                val.value = ValFormValue::Prim(prim);
            }
            FormParam::ValueSymbol(symbol) => {
                val.value = ValFormValue::ValueSymbol(symbol);
            }
            FormParam::Form(form) => match form.name.as_str() {
                "prod" => {
                    let form = ProdForm::from_form(&form)?;
                    val.value = ValFormValue::ProdForm(Box::new(form));
                }
                "fun" => {
                    let form = FunForm::from_form(&form)?;
                    val.value = ValFormValue::FunForm(Box::new(form));
                }
                "let" => {
                    let form = LetForm::from_form(&form)?;
                    val.value = ValFormValue::LetForm(Box::new(form));
                }
                "case" => {
                    let form = CaseForm::from_form(&form)?;
                    val.value = ValFormValue::CaseForm(Box::new(form));
                }
                _ => {
                    if let Ok(form) = AppForm::from_form(&form) {
                        val.value = ValFormValue::AppForm(Box::new(form));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form with types".to_string(),
                        }));
                    }
                }
            },
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: format!("unexpected value: {}", x.to_string()),
                }));
            }
        }

        Ok(val)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ValForm> {
        let form = Form::from_tokens(tokens)?;

        ValForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ValForm> {
        let tokens = Tokens::from_str(s)?;

        ValForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(val {} {})", self.name, self.value.to_string())
    }
}

impl fmt::Display for ValForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn val_form_from_str() {
        use super::ValForm;

        let mut s = "(val empty ())";

        let mut res = ValForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "empty".to_string());
        assert_eq!(form.value.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_empty_literal());
        assert!(form.is_value());

        s = "(val x 10)";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_primitive());
        assert!(form.is_value());

        s = "(val w x)";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "w".to_string());
        assert_eq!(form.value.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_value_symbol());
        assert!(form.is_value());

        s = "(val s (math.+ (prod 10.323 1)))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "s".to_string());
        assert_eq!(
            form.value.to_string(),
            "(math.+ (prod 10.323 1))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_application_form());
        assert!(form.is_value());

        s = "(val p (prod a b c d))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "p".to_string());
        assert_eq!(form.value.to_string(), "(prod a b c d)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_product_form());
        assert!(form.is_value());

        s = "(val p (prod a b (f (prod x y 10)) 11))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "p".to_string());
        assert_eq!(
            form.value.to_string(),
            "(prod a b (f (prod x y 10)) 11)".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_product_form());
        assert!(form.is_value());

        s = "(val err (let (type StringError String) (unwrap \"error\")))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "err".to_string());
        assert_eq!(
            form.value.to_string(),
            "(let (type StringError String) (unwrap \"error\"))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_let_form());
        assert!(form.is_value());

        s = "(val unwrap (fun res (case res (match T id) (match E panic))))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "unwrap".to_string());
        assert_eq!(
            form.value.to_string(),
            "(fun res (case res (match T id) (match E panic)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_function_form());
        assert!(form.is_value());
    }
}
