use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunFormBody {
    Prim(String),
    Symbol(String),
    FunApp(FunAppForm),
}

impl Default for FunFormBody {
    fn default() -> FunFormBody {
        FunFormBody::Prim("()".into())
    }
}

impl FunFormBody {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormBody::Prim(prim) => prim.clone(),
            FunFormBody::Symbol(symbol) => symbol.clone(),
            FunFormBody::FunApp(fun_app) => fun_app.to_string(),
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
    pub name: String,
    pub params: Vec<String>,
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

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<FunForm> {
        if fun_app.name != "fun" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a fun keyword".into(),
            }));
        }

        if fun_app.params.len() != 3 {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a name, a product of symbols and a function app form or a symbol"
                    .into(),
            }));
        }

        let mut fun = FunForm::new();
        fun.tokens = fun_app.tokens.clone();

        match fun_app.params[0].clone() {
            FunAppFormParam::Symbol(symbol) => {
                fun.name = symbol;
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: fun_app.loc(),
                    desc: "expected a symbol".into(),
                }));
            }
        }

        match fun_app.params[1].clone() {
            FunAppFormParam::Prim(prim) => {
                if prim != "()" {
                    return Err(Error::Semantic(SemanticError {
                        loc: fun_app.loc(),
                        desc: "expected an empty literal".into(),
                    }));
                }
            }
            FunAppFormParam::FunApp(form) => {
                if form.name != "prod" {
                    return Err(Error::Semantic(SemanticError {
                        loc: fun_app.loc(),
                        desc: "expected a product of symbols".into(),
                    }));
                }

                if form.params.len() < 2 {
                    return Err(Error::Semantic(SemanticError {
                        loc: fun_app.loc(),
                        desc: "expected at least one parameter".into(),
                    }));
                }

                if form.params[0].to_string() != "_" {
                    return Err(Error::Semantic(SemanticError {
                        loc: fun_app.loc(),
                        desc: "expected an anonymous product".into(),
                    }));
                }

                for param in form.params[1..].iter() {
                    match param {
                        FunAppFormParam::Symbol(symbol) => {
                            fun.params.push(symbol.clone());
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: fun_app.loc(),
                                desc: "expected a symbol".into(),
                            }));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: fun_app.loc(),
                    desc: "expected a product of symbols or an empty literal".into(),
                }));
            }
        }

        match fun_app.params[2].clone() {
            FunAppFormParam::Prim(prim) => {
                fun.body = FunFormBody::Prim(prim);
            }
            FunAppFormParam::Symbol(symbol) => {
                fun.body = FunFormBody::Symbol(symbol);
            }
            FunAppFormParam::FunApp(form) => {
                fun.body = FunFormBody::FunApp(form);
            }
        }

        Ok(fun)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<FunForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        FunForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<FunForm> {
        let tokens = Tokens::from_str(s)?;

        FunForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        let params = if self.params.is_empty() {
            "()".to_string()
        } else {
            format!(
                "(prod _ {})",
                self.params
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        };

        format!(
            "(fun {} {} {})",
            self.name.to_string(),
            params,
            self.body.to_string()
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

        let mut s = "(fun f1 () 10)";

        let mut res = FunForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "f1".to_string());
        assert!(form.params.is_empty());
        assert_eq!(form.body.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun f2 (prod _ a b c d) (+ a b c d 10))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "f2".to_string());
        assert_eq!(
            form.params,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ]
        );
        assert_eq!(form.body.to_string(), "(+ a b c d 10)".to_string());
    }
}
