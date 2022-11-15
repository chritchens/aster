use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::attrs_form::AttrsForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormParam};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::ProdForm;
use crate::form::type_form::TypeForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_qualified, is_type_symbol, is_value_symbol};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DefFormValue {
    Empty,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    AttrsForm(Box<AttrsForm>),
    TypeForm(Box<TypeForm>),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    CaseForm(Box<CaseForm>),
}

impl Default for DefFormValue {
    fn default() -> DefFormValue {
        DefFormValue::Empty
    }
}

impl DefFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            DefFormValue::Empty => "()".into(),
            DefFormValue::Prim(prim) => prim.clone(),
            DefFormValue::TypeKeyword(keyword) => keyword.clone(),
            DefFormValue::TypeSymbol(symbol) => symbol.clone(),
            DefFormValue::ValueSymbol(symbol) => symbol.clone(),
            DefFormValue::AttrsForm(form) => form.to_string(),
            DefFormValue::TypeForm(form) => form.to_string(),
            DefFormValue::ProdForm(form) => form.to_string(),
            DefFormValue::FunForm(form) => form.to_string(),
            DefFormValue::LetForm(form) => form.to_string(),
            DefFormValue::AppForm(form) => form.to_string(),
            DefFormValue::CaseForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for DefFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct DefForm {
    pub tokens: Box<Tokens>,
    pub name: String,
    pub value: DefFormValue,
}

impl DefForm {
    pub fn new() -> DefForm {
        DefForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn is_empty_literal(&self) -> bool {
        match self.value {
            DefFormValue::Empty => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self.value {
            DefFormValue::Prim(_) => true,
            _ => false,
        }
    }

    pub fn is_type_keyword(&self) -> bool {
        match self.value {
            DefFormValue::TypeKeyword(_) => true,
            _ => false,
        }
    }

    pub fn is_value_symbol(&self) -> bool {
        match self.value {
            DefFormValue::ValueSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_type_symbol(&self) -> bool {
        match self.value {
            DefFormValue::TypeSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_value_attributes_form(&self) -> bool {
        match self.value {
            DefFormValue::AttrsForm(_) => is_value_symbol(&self.name),
            _ => false,
        }
    }

    pub fn is_type_attributes_form(&self) -> bool {
        match self.value {
            DefFormValue::AttrsForm(_) => is_type_symbol(&self.name),
            _ => false,
        }
    }

    pub fn is_product_form(&self) -> bool {
        match self.value {
            DefFormValue::ProdForm(_) => true,
            _ => false,
        }
    }

    pub fn is_function_form(&self) -> bool {
        match self.value {
            DefFormValue::FunForm(_) => true,
            _ => false,
        }
    }

    pub fn is_application_form(&self) -> bool {
        match self.value {
            DefFormValue::AppForm(_) => true,
            _ => false,
        }
    }

    pub fn is_let_form(&self) -> bool {
        match self.value {
            DefFormValue::LetForm(_) => true,
            _ => false,
        }
    }

    pub fn is_type_form(&self) -> bool {
        match self.value {
            DefFormValue::TypeForm(_) => true,
            _ => false,
        }
    }

    pub fn is_case_form(&self) -> bool {
        match self.value {
            DefFormValue::CaseForm(_) => true,
            _ => false,
        }
    }

    pub fn is_attributes(&self) -> bool {
        self.is_type_attributes_form() || self.is_value_attributes_form()
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

    pub fn is_type(&self) -> bool {
        self.is_type_keyword()
            || self.is_type_symbol()
            || self.is_type_form()
            || (self.is_let_form() && is_type_symbol(&self.name))
            || (self.is_application_form() && is_type_symbol(&self.name))
    }

    pub fn from_form(form: &Form) -> Result<DefForm> {
        if form.name != "def" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a def keyword".into(),
            }));
        }

        if form.params.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and an empty literal or a primitive or a symbol or a form"
                    .into(),
            }));
        }

        let mut def = DefForm::new();
        def.tokens = form.tokens.clone();

        match form.params[0].clone() {
            FormParam::ValueSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                def.name = symbol;
            }
            FormParam::TypeSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                def.name = symbol;
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a value symbol or a type symbol".into(),
                }));
            }
        }

        match form.params[1].clone() {
            FormParam::Empty => {}
            FormParam::Prim(prim) => {
                def.value = DefFormValue::Prim(prim);
            }
            FormParam::TypeKeyword(keyword) => {
                def.value = DefFormValue::TypeKeyword(keyword);
            }
            FormParam::TypeSymbol(symbol) => {
                def.value = DefFormValue::TypeSymbol(symbol);
            }
            FormParam::ValueSymbol(symbol) => {
                def.value = DefFormValue::ValueSymbol(symbol);
            }
            FormParam::Form(form) => match form.name.as_str() {
                "attrs" => {
                    let form = AttrsForm::from_form(&form)?;
                    def.value = DefFormValue::AttrsForm(Box::new(form));
                }
                "prod" => {
                    let form = ProdForm::from_form(&form)?;
                    def.value = DefFormValue::ProdForm(Box::new(form));
                }
                "fun" => {
                    let form = FunForm::from_form(&form)?;
                    def.value = DefFormValue::FunForm(Box::new(form));
                }
                "let" => {
                    let form = LetForm::from_form(&form)?;
                    def.value = DefFormValue::LetForm(Box::new(form));
                }
                "case" => {
                    let form = CaseForm::from_form(&form)?;
                    def.value = DefFormValue::CaseForm(Box::new(form));
                }
                x => {
                    if is_type_symbol(x) {
                        if let Ok(form) = TypeForm::from_form(&form) {
                            def.value = DefFormValue::TypeForm(Box::new(form));
                        } else {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "unexpected mixed form".to_string(),
                            }));
                        }
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        def.value = DefFormValue::AppForm(Box::new(form));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected mixed form".to_string(),
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

        Ok(def)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<DefForm> {
        let form = Form::from_tokens(tokens)?;

        DefForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<DefForm> {
        let tokens = Tokens::from_str(s)?;

        DefForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(def {} {})", self.name, self.value.to_string())
    }
}

impl fmt::Display for DefForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn def_form_from_str() {
        use super::DefForm;

        let mut s = "(def empty ())";

        let mut res = DefForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "empty".to_string());
        assert_eq!(form.value.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_empty_literal());
        assert!(form.is_value());

        s = "(def x 10)";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_primitive());
        assert!(form.is_value());

        s = "(def w x)";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "w".to_string());
        assert_eq!(form.value.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_value_symbol());
        assert!(form.is_value());

        s = "(def s (math.+ (prod 10.323 1)))";

        res = DefForm::from_str(s);

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

        s = "(def p (prod a b c d))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "p".to_string());
        assert_eq!(form.value.to_string(), "(prod a b c d)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_product_form());
        assert!(form.is_value());

        s = "(def p (prod a b (f (prod x y 10)) 11))";

        res = DefForm::from_str(s);

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

        s = "(def C Char)";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "C".to_string());
        assert_eq!(form.value.to_string(), "Char".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_type_keyword());
        assert!(form.is_type());

        s = "(def Result (Sum T E))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Result".to_string());
        assert_eq!(form.value.to_string(), "(Sum T E)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_type_form());
        assert!(form.is_type());

        s = "(def err (let (def StringError String) (unwrap \"error\")))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "err".to_string());
        assert_eq!(
            form.value.to_string(),
            "(let (def StringError String) (unwrap \"error\"))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_let_form());
        assert!(form.is_value());

        s = "(def unwrap (fun res (case res (match T id) (match E panic))))";

        res = DefForm::from_str(s);

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
