use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_type_keyword};
use crate::syntax::{is_symbol, is_type_symbol, is_value_symbol, symbol_name};
use crate::token::{TokenKind, Tokens};
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum FormTailElement {
    Simple(SimpleValue),
    Form(Box<Form>),
}

impl FormTailElement {
    pub fn file(&self) -> String {
        match self {
            FormTailElement::Simple(value) => value.file(),
            FormTailElement::Form(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            FormTailElement::Simple(value) => value.loc(),
            FormTailElement::Form(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FormTailElement::Simple(value) => value.to_string(),
            FormTailElement::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for FormTailElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct Form {
    pub tokens: Box<Tokens>,
    pub head: SimpleValue,
    pub tail: Vec<FormTailElement>,
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

    pub fn tail_to_string(&self) -> String {
        self.tail
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn is_value_form(&self) -> bool {
        if !is_value_symbol(&symbol_name(&self.head.to_string())) {
            return false;
        }

        for param in self.tail.iter() {
            match param {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::TypeKeyword(_)
                    | SimpleValue::TypeSymbol(_)
                    | SimpleValue::TypePathSymbol(_) => {
                        return false;
                    }
                    _ => {}
                },
                FormTailElement::Form(form) => {
                    if !form.is_value_form() {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn is_types_form(&self) -> bool {
        let head = self.head.to_string();

        if !is_type_symbol(&symbol_name(&head)) && !is_type_keyword(&head) {
            return false;
        }

        for param in self.tail.iter() {
            match param {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::TypeKeyword(_)
                    | SimpleValue::TypeSymbol(_)
                    | SimpleValue::TypePathSymbol(_) => {}
                    _ => {
                        return false;
                    }
                },
                FormTailElement::Form(form) => {
                    if !form.is_types_form() {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn is_mixed_form(&self) -> bool {
        !(self.is_value_form() || self.is_types_form())
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

        let mut form = Form::new();
        form.tokens = Box::new(tokens.to_owned());

        let head_token = tokens[1].clone();
        let head = head_token.to_string();

        if !is_symbol(&symbol_name(&head)) && !is_keyword(&head) {
            return Err(Error::Syntactic(SyntacticError {
                loc: tokens[1].loc(),
                desc: "expected a symbol or a keyword".into(),
            }));
        }

        form.head = SimpleValue::from_token(&head_token)?;

        let mut idx = 2;

        while idx < len {
            match tokens[idx].kind {
                TokenKind::Comment | TokenKind::DocComment => {
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
                    form.tail.push(FormTailElement::Form(Box::new(inner_form)));
                }
                TokenKind::FormEnd => {
                    idx += 1;
                    break;
                }
                _ => {
                    let token = tokens[idx].clone();
                    let value = SimpleValue::from_token(&token)?;

                    form.tail.push(FormTailElement::Simple(value));

                    idx += 1;
                }
            }
        }

        if idx + 1 < len {
            return Err(Error::Syntactic(SyntacticError {
                loc: tokens[idx].loc(),
                desc: format!("unexpected token: {}", tokens[idx].to_string()),
            }));
        }

        Ok(form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Form> {
        let tokens = Tokens::from_str(s)?;

        Form::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "({} {})",
            self.head,
            self.tail
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

impl std::str::FromStr for Form {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
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

        assert_eq!(form.head.to_string(), "x.f".to_string());
        assert_eq!(form.tail_to_string(), "-1 T".to_string());
        assert!(form.is_mixed_form());

        s = "(x.f a 'b' 0)";

        res = Form::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.head.to_string(), "x.f".to_string());
        assert_eq!(form.tail_to_string(), "a 'b' 0".to_string());
        assert!(form.is_value_form());
        assert_eq!(form.to_string(), s.to_string());

        s = "(Fun T Q (Fun moduleA.A T Q B))";

        res = Form::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.head.to_string(), "Fun".to_string());
        assert_eq!(
            form.tail
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec![
                "T".to_string(),
                "Q".to_string(),
                "(Fun moduleA.A T Q B)".into()
            ]
        );
        assert_eq!(
            form.tail_to_string(),
            "T Q (Fun moduleA.A T Q B)".to_string()
        );
        assert!(form.is_types_form());

        s = "(Enum A B c.C Char)";

        res = Form::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.head.to_string(), "Enum".to_string());
        assert_eq!(
            form.tail
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>(),
            vec!["A".to_string(), "B".into(), "c.C".into(), "Char".into()]
        );
        assert_eq!(form.tail_to_string(), "A B c.C Char".to_string());
        assert!(form.is_types_form());
        assert_eq!(form.to_string(), s.to_string());
    }
}
