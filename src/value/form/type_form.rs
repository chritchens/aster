use super::{MixedAppForm, MixedAppFormParam, TypeAppForm};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeFormBody {
    Prim(String),
    Symbol(String),
    TypeApp(TypeAppForm),
}

impl Default for TypeFormBody {
    fn default() -> TypeFormBody {
        TypeFormBody::Prim("()".into())
    }
}

impl TypeFormBody {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypeFormBody::Prim(prim) => prim.clone(),
            TypeFormBody::Symbol(symbol) => symbol.clone(),
            TypeFormBody::TypeApp(type_app) => type_app.to_string(),
        }
    }
}

impl fmt::Display for TypeFormBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypeForm {
    pub tokens: Tokens,
    pub params: Vec<String>,
    pub body: TypeFormBody,
}

impl TypeForm {
    pub fn new() -> TypeForm {
        TypeForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<TypeForm> {
        if mixed_app.name != "type" {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a type keyword".into(),
            }));
        }

        if mixed_app.params.len() != 2 {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a product of symbols and a type application form or a type symbol"
                    .into(),
            }));
        }

        let mut type_form = TypeForm::new();
        type_form.tokens = mixed_app.tokens.clone();

        match mixed_app.params[0].clone() {
            MixedAppFormParam::Empty => {}
            MixedAppFormParam::MixedApp(form) => {
                if form.name != "prod" {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a product of symbols".into(),
                    }));
                }

                if form.params.is_empty() {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected at least one parameter".into(),
                    }));
                }

                for param in form.params.iter() {
                    match param {
                        MixedAppFormParam::TypeSymbol(symbol) => {
                            type_form.params.push(symbol.clone());
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: mixed_app.loc(),
                                desc: "expected a type symbol".into(),
                            }));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a product of symbols or an empty literal".into(),
                }));
            }
        }

        match mixed_app.params[1].clone() {
            MixedAppFormParam::TypeSymbol(symbol) => {
                type_form.body = TypeFormBody::Symbol(symbol);
            }
            MixedAppFormParam::TypeApp(form) => {
                type_form.body = TypeFormBody::TypeApp(form);
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a type application form or a type symbol".into(),
                }));
            }
        }

        Ok(type_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<TypeForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        TypeForm::from_mixed_app(&mixed_app)
    }

    pub fn from_str(s: &str) -> Result<TypeForm> {
        let tokens = Tokens::from_str(s)?;

        TypeForm::from_tokens(&tokens)
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

        format!("(type {} {})", params, self.body.to_string())
    }
}

impl fmt::Display for TypeForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_form_from_str() {
        use super::TypeForm;

        let mut s = "(type () Q)";

        let mut res = TypeForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert!(form.params.is_empty());
        assert_eq!(form.body.to_string(), "Q".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(type (prod A B C D) (Fun (Prod A B C D) String))";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.params,
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ]
        );
        assert_eq!(
            form.body.to_string(),
            "(Fun (Prod A B C D) String)".to_string()
        );
    }
}
