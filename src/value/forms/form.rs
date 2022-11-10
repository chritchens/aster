use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_symbol, is_type_symbol, is_value_symbol, symbol_name};
use crate::token::{TokenKind, Tokens};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FormParam {
    Ignore,
    Empty,
    Prim(String),
    ValueKeyword(String),
    TypeKeyword(String),
    ValueSymbol(String),
    TypeSymbol(String),
    Form(Box<Form>),
}

impl FormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FormParam::Ignore => "_".into(),
            FormParam::Empty => "()".into(),
            FormParam::Prim(prim) => prim.clone(),
            FormParam::ValueKeyword(keyword) => keyword.clone(),
            FormParam::TypeKeyword(keyword) => keyword.clone(),
            FormParam::ValueSymbol(symbol) => symbol.clone(),
            FormParam::TypeSymbol(symbol) => symbol.clone(),
            FormParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for FormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Form {
    pub tokens: Tokens,
    pub name: String,
    pub params: Vec<FormParam>,
}

impl Form {
    pub fn new() -> Form {
        Form::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn params_to_string(&self) -> String {
        self.params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn is_value_form(&self) -> bool {
        if !is_value_symbol(&symbol_name(&self.name)) {
            return false;
        }

        for param in self.params.iter() {
            match param {
                FormParam::TypeKeyword(_) | FormParam::TypeSymbol(_) => {
                    return false;
                }
                FormParam::Form(form) => {
                    if !form.is_value_form() {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn is_type_form(&self) -> bool {
        if !is_type_symbol(&symbol_name(&self.name)) {
            return false;
        }

        for param in self.params.iter() {
            match param {
                FormParam::Empty
                | FormParam::Prim(_)
                | FormParam::ValueKeyword(_)
                | FormParam::ValueSymbol(_) => {
                    return false;
                }
                FormParam::Form(form) => {
                    if !form.is_type_form() {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn is_mixed_form(&self) -> bool {
        !(self.is_value_form() || self.is_type_form())
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<Form> {
        let len = tokens.len();

        if tokens[0].kind != TokenKind::FormStart {
            return Err(Error::Syntactic(SyntacticError {
                loc: tokens[0].loc(),
                desc: "expected a form".into(),
            }));
        }

        if tokens[len - 1].kind != TokenKind::FormEnd {
            return Err(Error::Syntactic(SyntacticError {
                loc: tokens[len - 1].loc(),
                desc: "expected a form".into(),
            }));
        }

        if !is_symbol(&symbol_name(&tokens[1].to_string())) {
            return Err(Error::Syntactic(SyntacticError {
                loc: tokens[1].loc(),
                desc: "expected a symbol or a keyword".into(),
            }));
        }

        let name = tokens[1].to_string();

        let mut params = vec![];

        let mut idx = 2;

        while idx < len {
            match tokens[idx].kind {
                TokenKind::Comment | TokenKind::DocComment => {
                    idx += 1;
                }
                TokenKind::EmptyLiteral => {
                    params.push(FormParam::Empty);
                    idx += 1;
                }
                TokenKind::UIntLiteral
                | TokenKind::IntLiteral
                | TokenKind::FloatLiteral
                | TokenKind::CharLiteral
                | TokenKind::StringLiteral => {
                    params.push(FormParam::Prim(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::Keyword => {
                    let value = tokens[idx].to_string();

                    if is_type_symbol(&symbol_name(&value)) {
                        params.push(FormParam::TypeKeyword(value));
                    } else if value == "_" {
                        params.push(FormParam::Ignore);
                    } else {
                        params.push(FormParam::ValueKeyword(value));
                    }

                    idx += 1;
                }
                TokenKind::ValueSymbol => {
                    params.push(FormParam::ValueSymbol(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::TypeSymbol => {
                    params.push(FormParam::TypeSymbol(tokens[idx].to_string()));
                    idx += 1;
                }
                TokenKind::PathSymbol => {
                    let name = tokens[idx].to_string();
                    let unqualified = symbol_name(&name);

                    if is_keyword(&unqualified) {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: tokens[idx].loc(),
                            desc: "a path symbol cannot end with a keyword".into(),
                        }));
                    }

                    if is_type_symbol(&unqualified) {
                        params.push(FormParam::TypeSymbol(tokens[idx].to_string()));
                    } else if is_value_symbol(&unqualified) {
                        params.push(FormParam::ValueSymbol(tokens[idx].to_string()));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: tokens[idx].loc(),
                            desc: "expected a qualified type symbol or a qualified value symbol"
                                .into(),
                        }));
                    }

                    idx += 1;
                }
                TokenKind::FormStart => {
                    let mut count = 1;

                    let mut inner_tokens = Tokens::new();
                    inner_tokens.push(tokens[idx].clone());

                    idx += 1;

                    while idx < len {
                        let token = tokens[idx].clone();
                        inner_tokens.push(token.clone());
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

                    let inner_form = Form::from_tokens(&inner_tokens)?;
                    params.push(FormParam::Form(Box::new(inner_form)));
                }
                TokenKind::FormEnd => {
                    idx += 1;
                    break;
                }
            }
        }

        if idx + 1 < len {
            return Err(Error::Syntactic(SyntacticError {
                loc: tokens[idx].loc(),
                desc: format!("unexpected token: {}", tokens[idx].to_string()),
            }));
        }

        Ok(Form {
            tokens: tokens.clone(),
            name,
            params,
        })
    }

    pub fn from_str(s: &str) -> Result<Form> {
        let tokens = Tokens::from_str(s)?;

        Form::from_tokens(&tokens)
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

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn form_from_str() {
        use super::Form;

        let mut s = "(x.f -1 T)";

        let mut res = Form::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "x.f".to_string());
        assert_eq!(form.params_to_string(), "-1 T".to_string());
        assert!(form.is_mixed_form());

        s = "(x.f a 'b' 0)";

        res = Form::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x.f".to_string());
        assert_eq!(form.params_to_string(), "a 'b' 0".to_string());
        assert!(form.is_value_form());
        assert_eq!(form.to_string(), s.to_string());

        s = "(Fun (Prod T Q) (Fun (Prod moduleA.A T Q) B))";

        res = Form::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Fun".to_string());
        assert_eq!(
            form.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec![
                "(Prod T Q)".to_string(),
                "(Fun (Prod moduleA.A T Q) B)".into()
            ]
        );
        assert_eq!(
            form.params_to_string(),
            "(Prod T Q) (Fun (Prod moduleA.A T Q) B)".to_string()
        );
        assert!(form.is_type_form());

        s = "(Sum A B c.C Char)";

        res = Form::from_str(s);

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
        assert_eq!(form.params_to_string(), "A B c.C Char".to_string());
        assert!(form.is_type_form());
        assert_eq!(form.to_string(), s.to_string());
    }
}
