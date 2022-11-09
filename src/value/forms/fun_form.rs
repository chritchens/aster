use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_qualified;
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use crate::value::forms::prod_form::{ProdForm, ProdFormValue};
use crate::value::forms::type_form::TypeForm;
use crate::value::forms::value_form::ValueForm;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunFormParam {
    Empty,
    ValueSymbol(String),
    TypeSymbol(String),
}

impl Default for FunFormParam {
    fn default() -> FunFormParam {
        FunFormParam::Empty
    }
}

impl FunFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormParam::Empty => "()".into(),
            FunFormParam::ValueSymbol(symbol) => symbol.clone(),
            FunFormParam::TypeSymbol(symbol) => symbol.clone(),
        }
    }
}

impl fmt::Display for FunFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunFormBody {
    Empty,
    Prim(String),
    TypeKeyword(String),
    ValueSymbol(String),
    TypeSymbol(String),
    ValueForm(ValueForm),
    TypeForm(TypeForm),
    MixedForm(Form),
}

impl Default for FunFormBody {
    fn default() -> FunFormBody {
        FunFormBody::Empty
    }
}

impl FunFormBody {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormBody::Empty => "()".into(),
            FunFormBody::Prim(prim) => prim.clone(),
            FunFormBody::TypeKeyword(keyword) => keyword.clone(),
            FunFormBody::ValueSymbol(symbol) => symbol.clone(),
            FunFormBody::TypeSymbol(symbol) => symbol.clone(),
            FunFormBody::ValueForm(form) => form.to_string(),
            FunFormBody::TypeForm(form) => form.to_string(),
            FunFormBody::MixedForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for FunFormBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FunForm {
    pub tokens: Tokens,
    pub params: Vec<FunFormParam>,
    pub body: FunFormBody,
}

impl FunForm {
    pub fn new() -> FunForm {
        FunForm::default()
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

    pub fn from_form(form: &Form) -> Result<FunForm> {
        if form.name != "fun" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a fun keyword".into(),
            }));
        }

        if form.params.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a symbol or form and a primitive, or a symbol or a form".into(),
            }));
        }

        let mut fun = FunForm::new();
        fun.tokens = form.tokens.clone();

        match form.params[0].clone() {
            FormParam::Empty => {}
            FormParam::ValueSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                fun.params.push(FunFormParam::ValueSymbol(symbol));
            }
            FormParam::TypeSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                fun.params.push(FunFormParam::TypeSymbol(symbol));
            }
            FormParam::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value {
                            ProdFormValue::TypeSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                fun.params.push(FunFormParam::TypeSymbol(symbol.clone()));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                fun.params.push(FunFormParam::ValueSymbol(symbol.clone()));
                            }
                            _ => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected a product of symbols".into(),
                                }));
                            }
                        }
                    }
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a product of symbols".into(),
                    }));
                }
            }
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: format!("unexpected function params: {}", x.to_string()),
                }));
            }
        }

        match form.params[1].clone() {
            FormParam::Empty => {
                fun.body = FunFormBody::Empty;
            }
            FormParam::Prim(prim) => {
                fun.body = FunFormBody::Prim(prim);
            }
            FormParam::TypeKeyword(keyword) => {
                fun.body = FunFormBody::TypeKeyword(keyword);
            }
            FormParam::ValueSymbol(symbol) => {
                fun.body = FunFormBody::ValueSymbol(symbol);
            }
            FormParam::TypeSymbol(symbol) => {
                fun.body = FunFormBody::TypeSymbol(symbol);
            }
            FormParam::Form(form) => {
                if form.is_value_form() {
                    let form = ValueForm::from_form(&form)?;
                    fun.body = FunFormBody::ValueForm(form);
                } else if form.is_type_form() {
                    let form = TypeForm::from_form(&form)?;
                    fun.body = FunFormBody::TypeForm(form);
                } else {
                    fun.body = FunFormBody::MixedForm(form);
                }
            }
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: format!("unexpected function body: {}", x.to_string()),
                }));
            }
        }

        Ok(fun)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<FunForm> {
        let form = Form::from_tokens(tokens)?;

        FunForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<FunForm> {
        let tokens = Tokens::from_str(s)?;

        FunForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(fun {} {})",
            self.params_to_string(),
            self.body.to_string(),
        )
    }
}

impl fmt::Display for FunForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fun_form_from_str() {
        use super::FunForm;
        use super::FunFormParam;

        let mut s = "(fun () x)";

        let mut res = FunForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.params, vec![]);
        assert_eq!(form.params_to_string(), "()".to_string());
        assert_eq!(form.body.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun x ())";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.params,
            vec![FunFormParam::ValueSymbol("x".to_string())]
        );
        assert_eq!(form.params_to_string(), "x".to_string());
        assert_eq!(form.body.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun x moduleX.x)";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.params,
            vec![FunFormParam::ValueSymbol("x".to_string())]
        );
        assert_eq!(form.params_to_string(), "x".to_string());
        assert_eq!(form.body.to_string(), "moduleX.x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun (prod a b c d) (math.+ a b 10 (math.* c d 10)))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params_to_string(), "(prod a b c d)".to_string());
        assert_eq!(
            form.body.to_string(),
            "(math.+ a b 10 (math.* c d 10))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
