use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_value_symbol, symbol_name};
use crate::token::{TokenKind, Tokens};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunAppFormParam {
    Empty,
    Prim(String),
    Symbol(String),
    App(FunAppForm),
}

impl FunAppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunAppFormParam::Empty => "()".into(),
            FunAppFormParam::Prim(prim) => prim.clone(),
            FunAppFormParam::Symbol(symbol) => symbol.clone(),
            FunAppFormParam::App(app) => app.to_string(),
        }
    }
}

impl fmt::Display for FunAppFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FunAppForm {
    pub tokens: Tokens,
    pub name: String,
    pub params: Vec<FunAppFormParam>,
}

impl FunAppForm {
    pub fn new() -> FunAppForm {
        FunAppForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<FunAppForm> {
        let len = tokens.len();

        if tokens[0].kind != TokenKind::FormStart {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[0].loc(),
                desc: "expected a form".into(),
            }));
        }

        if tokens[len - 1].kind != TokenKind::FormEnd {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[len - 1].loc(),
                desc: "expected a form".into(),
            }));
        }

        if !is_value_symbol(&symbol_name(&tokens[1].to_string())) {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "expected a value symbol or a value keyword".into(),
            }));
        }

        let name = tokens[1].to_string();

        if is_keyword(&name) && name == "_" {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "unexpected wildcard keyword".into(),
            }));
        }

        let mut params = vec![];

        let mut idx = 2;

        while idx < len {
            match tokens[idx].kind {
                TokenKind::EmptyLiteral => {
                    params.push(FunAppFormParam::Empty);
                    idx += 1;
                }
                TokenKind::UIntLiteral
                | TokenKind::IntLiteral
                | TokenKind::FloatLiteral
                | TokenKind::CharLiteral
                | TokenKind::StringLiteral => {
                    params.push(FunAppFormParam::Prim(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::ValueSymbol => {
                    params.push(FunAppFormParam::Symbol(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::PathSymbol => {
                    let value = tokens[idx].to_string();
                    let unqualified = symbol_name(&value);

                    if is_keyword(&unqualified) {
                        return Err(Error::Semantic(SemanticError {
                            loc: tokens[idx].loc(),
                            desc: "a path symbol cannot end with a keyword".into(),
                        }));
                    }

                    params.push(FunAppFormParam::Symbol(value));
                    idx += 1;
                }
                TokenKind::FormStart => {
                    let mut count = 1;

                    let mut new_tokens = Tokens::new();
                    new_tokens.push(tokens[idx].clone());

                    idx += 1;

                    while idx < len {
                        let token = tokens[idx].clone();
                        new_tokens.push(token.clone());
                        idx += 1;

                        if token.kind == TokenKind::FormStart {
                            count += 1;
                        } else if token.kind == TokenKind::FormEnd {
                            count -= 1;

                            if count == 0 {
                                break;
                            }
                        }
                    }

                    let app = FunAppForm::from_tokens(&new_tokens)?;

                    params.push(FunAppFormParam::App(app));
                }
                TokenKind::FormEnd => {
                    idx += 1;
                    break;
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: tokens[idx].loc(),
                        desc: format!("unexpected token: {}", tokens[idx].to_string()),
                    }));
                }
            }
        }

        if idx + 1 < len {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[idx].loc(),
                desc: format!("unexpected token: {}", tokens[idx].to_string()),
            }));
        }

        Ok(FunAppForm {
            tokens: tokens.clone(),
            name,
            params,
        })
    }

    pub fn from_str(s: &str) -> Result<FunAppForm> {
        let tokens = Tokens::from_str(s)?;

        FunAppForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "({} {})",
            self.name,
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for FunAppForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fun_app_form_from_str() {
        use super::FunAppForm;

        let mut s = "(f a 10)";

        let mut res = FunAppForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "f".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["a".to_string(), "10".to_string()]
        );

        s = "(app (moduleX.getFunc \"f\") x y)";

        res = FunAppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "app".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec![
                "(moduleX.getFunc \"f\")".to_string(),
                "x".into(),
                "y".into()
            ]
        );
    }
}
