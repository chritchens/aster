use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::ProdForm;
use crate::form::simple_value::SimpleValue;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_value_symbol;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValFormValue {
    Empty(SimpleValue),
    Panic(SimpleValue),
    Prim(SimpleValue),
    ValueSymbol(SimpleValue),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    CaseForm(Box<CaseForm>),
}

impl Default for ValFormValue {
    fn default() -> ValFormValue {
        ValFormValue::Empty(SimpleValue::new())
    }
}

impl ValFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ValFormValue::Empty(_) => "()".into(),
            ValFormValue::Panic(_) => "panic".into(),
            ValFormValue::Prim(prim) => prim.to_string(),
            ValFormValue::ValueSymbol(symbol) => symbol.to_string(),
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
    pub name: SimpleValue,
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
            ValFormValue::Empty(_) => true,
            _ => false,
        }
    }

    pub fn is_panic(&self) -> bool {
        match self.value {
            ValFormValue::Panic(_) => true,
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
            || (self.is_let_form() && is_value_symbol(&self.name.to_string()))
            || (self.is_application_form() && is_value_symbol(&self.name.to_string()))
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];
        params.push(self.name.clone());

        match self.value.clone() {
            ValFormValue::ProdForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.value.clone() {
            ValFormValue::ValueSymbol(value) => {
                vars.push(value);
            }
            ValFormValue::ProdForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::CaseForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<ValForm> {
        if form.head.to_string() != "val" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a val keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and an empty literal or a primitive or a symbol or a form"
                    .into(),
            }));
        }

        let mut val = ValForm::new();
        val.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    val.name = value;
                }
                SimpleValue::TypeSymbol(_) => {
                    val.name = value;
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a type or value symbol".into(),
                    }));
                }
            },
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a type or value symbol".into(),
                }));
            }
        }

        match form.tail[1].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(empty) => {
                    val.value = ValFormValue::Empty(SimpleValue::Empty(empty));
                }
                SimpleValue::Panic(empty) => {
                    val.value = ValFormValue::Panic(SimpleValue::Panic(empty));
                }
                SimpleValue::Prim(prim) => {
                    val.value = ValFormValue::Prim(SimpleValue::Prim(prim));
                }
                SimpleValue::ValueSymbol(symbol) => {
                    val.value = ValFormValue::ValueSymbol(SimpleValue::ValueSymbol(symbol));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected value: {}", x.to_string()),
                    }));
                }
            },

            FormTailElement::Form(form) => match form.head.to_string().as_str() {
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

impl std::str::FromStr for ValForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
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

        assert_eq!(form.name.to_string(), "empty".to_string());
        assert_eq!(form.value.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_empty_literal());
        assert!(form.is_value());

        s = "(val x 10)";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_primitive());
        assert!(form.is_value());

        s = "(val w x)";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "w".to_string());
        assert_eq!(form.value.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_value_symbol());
        assert!(form.is_value());

        s = "(val s (math.+ (prod 10.323 1)))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "s".to_string());
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

        assert_eq!(form.name.to_string(), "p".to_string());
        assert_eq!(form.value.to_string(), "(prod a b c d)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_product_form());
        assert!(form.is_value());

        s = "(val p (prod a b (f (prod x y 10)) 11))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "p".to_string());
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

        assert_eq!(form.name.to_string(), "err".to_string());
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

        assert_eq!(form.name.to_string(), "unwrap".to_string());
        assert_eq!(
            form.value.to_string(),
            "(fun res (case res (match T id) (match E panic)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_function_form());
        assert!(form.is_value());
    }
}
