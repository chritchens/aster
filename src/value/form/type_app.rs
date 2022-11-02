use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_type_symbol;
use crate::token::{TokenKind, Tokens};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeAppFormParam {
    TypeSymbol(String),
    TypeApp(TypeAppForm),
}

impl TypeAppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypeAppFormParam::TypeSymbol(symbol) => symbol.clone(),
            TypeAppFormParam::TypeApp(type_app) => type_app.to_string(),
        }
    }
}

impl fmt::Display for TypeAppFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypeAppForm {
    pub tokens: Tokens,
    pub name: String,
    pub params: Vec<TypeAppFormParam>,
}

impl TypeAppForm {
    pub fn new() -> TypeAppForm {
        TypeAppForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<TypeAppForm> {
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

        if !is_type_symbol(&tokens[1].to_string()) {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "expected a type symbol or a type keyword".into(),
            }));
        }

        let name = tokens[1].to_string();
        let mut params = vec![];

        let mut idx = 2;

        while idx < len {
            match tokens[idx].kind {
                TokenKind::TypeSymbol => {
                    params.push(TypeAppFormParam::TypeSymbol(tokens[idx].to_string()));
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

                    let type_app = TypeAppForm::from_tokens(&new_tokens)?;

                    params.push(TypeAppFormParam::TypeApp(type_app));
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

        Ok(TypeAppForm {
            tokens: tokens.clone(),
            name,
            params,
        })
    }

    pub fn from_str(s: &str) -> Result<TypeAppForm> {
        let tokens = Tokens::from_str(s)?;

        TypeAppForm::from_tokens(&tokens)
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

impl fmt::Display for TypeAppForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_app_form_from_str() {
        use super::TypeAppForm;

        let mut s = "(Fun A B)";

        let mut res = TypeAppForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "Fun".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["A".to_string(), "B".to_string()]
        );

        s = "(App (Fun A B E) C D)";

        res = TypeAppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "App".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["(Fun A B E)".to_string(), "C".into(), "D".into()]
        );
    }
}
