use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::token::{TokenKind, Tokens};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ExportForm {
    pub tokens: Tokens,
    pub type_defs: Vec<String>,
    pub value_defs: Vec<String>,
}

impl ExportForm {
    pub fn new() -> ExportForm {
        ExportForm::default()
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ExportForm> {
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

        if tokens[1].to_string() != "export" {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[1].loc(),
                desc: "expected an export form".into(),
            }));
        }

        let mut type_defs = vec![];
        let mut value_defs = vec![];
        let mut idx = 2;

        match tokens[idx].kind {
            TokenKind::TypeSymbol => {
                type_defs.push(tokens[idx].to_string());
                idx += 1;
            }
            TokenKind::ValueSymbol => {
                value_defs.push(tokens[idx].to_string());
                idx += 1;
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

        if idx + 1 < len {
            return Err(Error::Semantic(SemanticError {
                loc: tokens[idx].loc(),
                desc: format!("unexpected token: {}", tokens[idx].to_string()),
            }));
        }

        Ok(ExportForm {
            tokens: tokens.clone(),
            type_defs,
            value_defs,
        })
    }

    pub fn from_str(s: &str) -> Result<ExportForm> {
        let tokens = Tokens::from_str(s)?;

        ExportForm::from_tokens(&tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn export_form_from_str() {
        use super::ExportForm;

        let mut s = "(export A)";

        let mut res = ExportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.type_defs, vec!["A".to_string()]);

        s = "(export (prod b C d E))";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.type_defs, vec!["C".to_string(), "E".to_string()]);
        assert_eq!(form.value_defs, vec!["b".to_string(), "d".to_string()]);
    }
}
