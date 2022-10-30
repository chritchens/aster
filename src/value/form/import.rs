use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::token::{TokenKind, Tokens};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ImportForm {
    pub tokens: Tokens,
    pub module: String,
    pub qualifier: Option<String>,
    pub type_defs: Vec<String>,
    pub value_defs: Vec<String>,
}

impl ImportForm {
    pub fn new() -> ImportForm {
        ImportForm::default()
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ImportForm> {
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

        if tokens[1].to_string() != "import" {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "expected an import form".into(),
            }));
        }

        if tokens[2].kind != TokenKind::PathSymbol {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[2].loc(),
                desc: "expected a module path".into(),
            }));
        }

        let module = tokens[2].to_string();

        let mut type_defs = vec![];
        let mut value_defs = vec![];
        let mut qualifier = None;
        let mut idx = 3;

        match tokens[idx].kind {
            TokenKind::ValueSymbol => {
                qualifier = Some(tokens[idx].to_string());
            }
            TokenKind::FormStart => {
                idx += 1;

                if tokens[idx].to_string() != "prod" {
                    return Err(Error::Semantic(SemanticError {
                        loc: tokens[idx].loc(),
                        desc: "expected a product form".into(),
                    }));
                }

                idx += 1;

                let start_idx = idx;

                for tidx in start_idx..len {
                    let token = tokens[tidx].clone();
                    idx += 1;

                    match token.kind {
                        TokenKind::FormEnd => {
                            break;
                        }
                        TokenKind::TypeSymbol => {
                            type_defs.push(token.to_string());
                        }
                        TokenKind::ValueSymbol => {
                            value_defs.push(token.to_string());
                        }
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: token.loc(),
                                desc: format!("unexpected token: {}", token.to_string()),
                            }));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: tokens[idx].loc(),
                    desc: format!("unexpected token: {}", tokens[idx].to_string()),
                }));
            }
        }

        if qualifier.is_some() && idx + 1 < len {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[idx].loc(),
                desc: format!("unexpected token: {}", tokens[idx].to_string()),
            }));
        } else if tokens[idx].kind == TokenKind::ValueSymbol {
            qualifier = Some(tokens[idx].to_string());
        }

        Ok(ImportForm {
            tokens: tokens.clone(),
            module,
            qualifier,
            type_defs,
            value_defs,
        })
    }

    pub fn from_str(s: &str) -> Result<ImportForm> {
        let tokens = Tokens::from_str(s)?;

        ImportForm::from_tokens(&tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn import_form_from_str() {
        use super::ImportForm;

        let s = "(import std.x (prod a B c D) x)";

        let res = ImportForm::from_str(s);

        assert!(res.is_ok());

        let form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_defs, vec!["B".to_string(), "D".to_string()]);
        assert_eq!(form.value_defs, vec!["a".to_string(), "c".to_string()]);
    }
}
