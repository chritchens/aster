use super::{FunAppForm, FunAppFormParam};
use super::{TypeAppForm, TypeAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_symbol, is_type_symbol, is_value_symbol, symbol_name};
use crate::token::{TokenKind, Tokens};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MixedAppFormParam {
    Empty,
    Prim(String),
    ValueSymbol(String),
    TypeSymbol(String),
    FunApp(FunAppForm),
    TypeApp(TypeAppForm),
    MixedApp(MixedAppForm),
}

impl MixedAppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            MixedAppFormParam::Empty => "()".into(),
            MixedAppFormParam::Prim(prim) => prim.clone(),
            MixedAppFormParam::ValueSymbol(symbol) => symbol.clone(),
            MixedAppFormParam::TypeSymbol(symbol) => symbol.clone(),
            MixedAppFormParam::FunApp(fun_app) => fun_app.to_string(),
            MixedAppFormParam::TypeApp(type_app) => type_app.to_string(),
            MixedAppFormParam::MixedApp(mixed_app) => mixed_app.to_string(),
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

    pub fn is_fun_app(&self) -> bool {
        if !is_value_symbol(&symbol_name(&self.name)) {
            return false;
        }

        for param in self.params.iter() {
            match param {
                MixedAppFormParam::TypeSymbol(_)
                | MixedAppFormParam::TypeApp(_)
                | MixedAppFormParam::MixedApp(_) => {
                    return false;
                }
                _ => {}
            }
        }

        true
    }

    pub fn is_type_app(&self) -> bool {
        if !is_type_symbol(&symbol_name(&self.name)) {
            return false;
        }

        for param in self.params.iter() {
            match param {
                MixedAppFormParam::Empty
                | MixedAppFormParam::Prim(_)
                | MixedAppFormParam::ValueSymbol(_)
                | MixedAppFormParam::FunApp(_)
                | MixedAppFormParam::MixedApp(_) => {
                    return false;
                }
                _ => {}
            }
        }

        true
    }

    pub fn is_mixed_app(&self) -> bool {
        !(self.is_fun_app() || self.is_type_app())
    }

    pub fn as_fun_app(&self) -> Option<FunAppForm> {
        if !self.is_fun_app() {
            return None;
        }

        let mut fun_app = FunAppForm::new();
        fun_app.name = self.name.clone();

        for param in self.params.clone() {
            match param {
                MixedAppFormParam::Empty => {
                    fun_app.params.push(FunAppFormParam::Empty);
                }
                MixedAppFormParam::Prim(prim) => {
                    fun_app.params.push(FunAppFormParam::Prim(prim));
                }
                MixedAppFormParam::ValueSymbol(symbol) => {
                    fun_app.params.push(FunAppFormParam::Symbol(symbol));
                }
                MixedAppFormParam::FunApp(app) => {
                    fun_app.params.push(FunAppFormParam::App(app));
                }
                _ => unreachable!(),
            }
        }

        Some(fun_app)
    }

    pub fn as_type_app(&self) -> Option<TypeAppForm> {
        if !self.is_type_app() {
            return None;
        }

        let mut type_app = TypeAppForm::new();
        type_app.name = self.name.clone();

        for param in self.params.clone() {
            match param {
                MixedAppFormParam::TypeSymbol(symbol) => {
                    type_app.params.push(TypeAppFormParam::Symbol(symbol));
                }
                MixedAppFormParam::TypeApp(app) => {
                    type_app.params.push(TypeAppFormParam::App(app));
                }
                _ => unreachable!(),
            }
        }

        Some(type_app)
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

        if !is_symbol(&symbol_name(&tokens[1].to_string())) {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "expected a symbol or a keyword".into(),
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
                    params.push(MixedAppFormParam::Empty);
                    idx += 1;
                }
                TokenKind::UIntLiteral
                | TokenKind::IntLiteral
                | TokenKind::FloatLiteral
                | TokenKind::CharLiteral
                | TokenKind::StringLiteral => {
                    params.push(MixedAppFormParam::Prim(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::ValueSymbol => {
                    params.push(MixedAppFormParam::ValueSymbol(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::TypeSymbol => {
                    params.push(MixedAppFormParam::TypeSymbol(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::PathSymbol => {
                    let name = tokens[idx].to_string();
                    let unqualified = symbol_name(&name);

                    if is_type_symbol(&unqualified) {
                        params.push(MixedAppFormParam::TypeSymbol(tokens[idx].to_string()));
                    } else if is_value_symbol(&unqualified) {
                        params.push(MixedAppFormParam::ValueSymbol(tokens[idx].to_string()));
                    } else {
                        return Err(Error::Semantic(SemanticError {
                            loc: tokens[idx].loc(),
                            desc: "expected a qualified type symbol or a qualified value symbol"
                                .into(),
                        }));
                    }

                    idx += 1;
                }
                TokenKind::Keyword => {
                    let value = tokens[idx].to_string();

                    if is_type_symbol(&symbol_name(&value)) {
                        params.push(MixedAppFormParam::TypeSymbol(value));
                    } else {
                        return Err(Error::Semantic(SemanticError {
                            loc: tokens[idx].loc(),
                            desc: "expected a type keyword".into(),
                        }));
                    }

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
                            if !is_type_app {
                                is_type_app = true;
                            }
                        } else if token.kind == TokenKind::ValueSymbol {
                            if !is_fun_app {
                                is_fun_app = true;
                            }
                        } else if token.kind == TokenKind::PathSymbol {
                            let name = symbol_name(&token.to_string());

                            if is_type_symbol(&name) {
                                if !is_type_app {
                                    is_type_app = true;
                                }
                            } else if !is_type_app {
                                is_fun_app = true;
                            }
                        } else if token.kind == TokenKind::Keyword {
                            let name = token.to_string();

                            if is_type_symbol(&name) {
                                if !is_type_app {
                                    is_type_app = true;
                                }
                            } else if !is_type_app {
                                is_fun_app = true;
                            }
                        }
                    }

                    if is_fun_app && is_type_app {
                        let mixed_app = MixedAppForm::from_tokens(&new_tokens)?;
                        params.push(MixedAppFormParam::MixedApp(mixed_app));
                    } else if is_fun_app && !is_type_app {
                        let fun_app = FunAppForm::from_tokens(&new_tokens)?;
                        params.push(MixedAppFormParam::FunApp(fun_app));
                    } else if is_type_app && !is_fun_app {
                        let type_app = TypeAppForm::from_tokens(&new_tokens)?;
                        params.push(MixedAppFormParam::TypeApp(type_app));
                    } else {
                        println!("new_tokens: {:?}", new_tokens);
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
        assert!(form.is_mixed_app());

        s = "(x.f a 'b' 0)";

        res = MixedAppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x.f".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["a".to_string(), "'b'".to_string(), "0".to_string()]
        );
        assert!(form.is_fun_app());
        assert_eq!(form.to_string(), s.to_string());
        assert_eq!(form.as_fun_app().unwrap().to_string(), s.to_string());

        s = "(type T Q (Fun moduleA.A T Q B))";

        res = MixedAppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "type".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["T".to_string(), "Q".into(), "(Fun moduleA.A T Q B)".into()]
        );
        assert!(form.is_mixed_app());

        s = "(Sum A B c.C Char)";

        res = MixedAppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Sum".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["A".to_string(), "B".into(), "c.C".into(), "Char".into()]
        );
        assert!(form.is_type_app());
        assert_eq!(form.to_string(), s.to_string());
        assert_eq!(form.as_type_app().unwrap().to_string(), s.to_string());
    }
}
