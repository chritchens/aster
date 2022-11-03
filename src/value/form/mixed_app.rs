use super::{FunAppForm, TypeAppForm};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::WILDCARD;
use crate::syntax::{is_symbol, is_type_symbol, symbol_name};
use crate::token::{TokenKind, Tokens};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MixedAppFormParam {
    Prim(String),
    Symbol(String),
    FunApp(FunAppForm),
    TypeApp(TypeAppForm),
}

impl MixedAppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            MixedAppFormParam::Prim(prim) => prim.clone(),
            MixedAppFormParam::Symbol(symbol) => symbol.clone(),
            MixedAppFormParam::FunApp(fun_app) => fun_app.to_string(),
            MixedAppFormParam::TypeApp(type_app) => type_app.to_string(),
        }
    }
}

impl fmt::Display for MixedAppFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct MixedAppForm {
    pub tokens: Tokens,
    pub name: String,
    pub params: Vec<MixedAppFormParam>,
}

impl MixedAppForm {
    pub fn new() -> MixedAppForm {
        MixedAppForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<MixedAppForm> {
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

        if !is_symbol(&tokens[1].to_string()) {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "expected a symbol or a keyword".into(),
            }));
        }

        let name = tokens[1].to_string();
        let mut params = vec![];

        let mut idx = 2;

        while idx < len {
            match tokens[idx].kind {
                TokenKind::EmptyLiteral
                | TokenKind::UIntLiteral
                | TokenKind::IntLiteral
                | TokenKind::FloatLiteral
                | TokenKind::CharLiteral
                | TokenKind::StringLiteral => {
                    params.push(MixedAppFormParam::Prim(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::ValueSymbol | TokenKind::TypeSymbol | TokenKind::PathSymbol => {
                    params.push(MixedAppFormParam::Symbol(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::Keyword => {
                    let value = tokens[idx].to_string();

                    if value != WILDCARD.to_string() {
                        return Err(Error::Semantic(SemanticError {
                            loc: tokens[idx].loc(),
                            desc: "expected the wildcard keyword".into(),
                        }));
                    }

                    params.push(MixedAppFormParam::Symbol(value));
                    idx += 1;
                }
                TokenKind::FormStart => {
                    let mut count = 1;
                    let mut is_type_app = false;
                    let mut is_fun_app = false;

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
                        } else if token.kind == TokenKind::TypeSymbol {
                            if is_fun_app {
                                return Err(Error::Semantic(SemanticError {
                                    loc: tokens[idx].loc(),
                                    desc: format!(
                                        "unexpected type symbol {}",
                                        tokens[idx].to_string()
                                    ),
                                }));
                            } else if !is_type_app {
                                is_type_app = true;
                            }
                        } else if token.kind == TokenKind::ValueSymbol {
                            if is_type_app {
                                return Err(Error::Semantic(SemanticError {
                                    loc: tokens[idx].loc(),
                                    desc: format!(
                                        "unexpected value symbol {}",
                                        tokens[idx].to_string()
                                    ),
                                }));
                            } else if !is_fun_app {
                                is_fun_app = true;
                            }
                        } else if token.kind == TokenKind::PathSymbol {
                            let name = symbol_name(&token.to_string());

                            if is_type_symbol(&name) {
                                if is_fun_app {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: tokens[idx].loc(),
                                        desc: format!(
                                            "unexpected type symbol {}",
                                            tokens[idx].to_string()
                                        ),
                                    }));
                                } else if !is_type_app {
                                    is_type_app = true;
                                }
                            } else if is_type_app {
                                return Err(Error::Semantic(SemanticError {
                                    loc: tokens[idx].loc(),
                                    desc: format!(
                                        "unexpected value symbol {}",
                                        tokens[idx].to_string()
                                    ),
                                }));
                            } else if !is_fun_app {
                                is_fun_app = true;
                            }
                        }
                    }

                    if is_fun_app {
                        let fun_app = FunAppForm::from_tokens(&new_tokens)?;
                        params.push(MixedAppFormParam::FunApp(fun_app));
                    } else {
                        let fun_app = TypeAppForm::from_tokens(&new_tokens)?;
                        params.push(MixedAppFormParam::TypeApp(fun_app));
                    }
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

        Ok(MixedAppForm {
            tokens: tokens.clone(),
            name,
            params,
        })
    }

    pub fn from_str(s: &str) -> Result<MixedAppForm> {
        let tokens = Tokens::from_str(s)?;

        MixedAppForm::from_tokens(&tokens)
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

impl fmt::Display for MixedAppForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mixed_app_form_from_str() {
        use super::MixedAppForm;

        let mut s = "(x.f -1 T)";

        let mut res = MixedAppForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "x.f".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["-1".to_string(), "T".to_string()]
        );

        s = "(type T Q (Fun A T Q B))";

        res = MixedAppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "type".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["T".to_string(), "Q".into(), "(Fun A T Q B)".into()]
        );
    }
}
