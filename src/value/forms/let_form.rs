use crate::error::{Error, SyntacticError};
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::app_form::AppForm;
use crate::value::forms::def_form::DefForm;
use crate::value::forms::form::{Form, FormParam};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct LetForm {
    pub defs: Vec<DefForm>,
    pub value: AppForm,
}

impl LetForm {
    pub fn new() -> LetForm {
        LetForm::default()
    }

    pub fn defs_to_string(&self) -> String {
        if self.defs.is_empty() {
            "".to_string()
        } else {
            self.defs
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        }
    }

    pub fn from_form(form: &Form) -> Result<LetForm> {
        if form.name != "let" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a let keyword".into(),
            }));
        }

        let len = form.params.len();

        if len == 0 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a function application".into(),
            }));
        }

        let mut let_form = LetForm::new();

        if len == 1 {
            match form.params[0].clone() {
                FormParam::Form(form) => {
                    let form = AppForm::from_form(&form)?;
                    let_form.value = form;
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a function application".into(),
                    }));
                }
            }
        }

        if len > 1 {
            for param in form.params[0..(len - 1)].iter().clone() {
                match param {
                    FormParam::Form(form) => {
                        if let Ok(form) = DefForm::from_form(&form) {
                            let_form.defs.push(form);
                        } else {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a definition form".into(),
                            }));
                        }
                    }
                    _ => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected a form".into(),
                        }));
                    }
                }
            }

            match form.params[len - 1].clone() {
                FormParam::Form(form) => {
                    if let Ok(form) = AppForm::from_form(&form) {
                        let_form.value = form;
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected an application form".into(),
                        }));
                    }
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a form".into(),
                    }));
                }
            }
        }

        Ok(let_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<LetForm> {
        let form = Form::from_tokens(tokens)?;

        LetForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<LetForm> {
        let tokens = Tokens::from_str(s)?;

        LetForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        if self.defs.is_empty() {
            format!("(let {})", self.value.to_string(),)
        } else {
            format!("(let {} {})", self.defs_to_string(), self.value.to_string(),)
        }
    }
}

impl fmt::Display for LetForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn let_form_from_str() {
        use super::LetForm;

        let mut s = "(let (math.exp (prod math.e 10)))";

        let mut res = LetForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert!(form.defs.is_empty());
        assert_eq!(
            form.value.to_string(),
            "(math.exp (prod math.e 10))".to_string()
        );
        assert_eq!(
            form.to_string(),
            "(let (math.exp (prod math.e 10)))".to_string()
        );

        s = "
        (let
            (def Result (attrs union))
            (def Result (prod T E) (Sum T E))

            (def unwrap (attrs inline))
            (def unwrap (prod T E) (Fun (Result T E) T))
            (def unwrap (fun res (case res (match T id) (match E (panic E)))))

            (def StringError String)
            (def StringResult (Result String StringResult))

            (def res String)
            (def res (unwrap (prod String StringError) \"res\"))
            (def res2 (unwrap _ \"res2\"))

            # return as a synonym of `id`
            (return (prod res res2)))";

        res = LetForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.defs.len(), 10);
        assert_eq!(
            form.defs[0].to_string(),
            "(def Result (attrs union))".to_string()
        );
        assert!(form.defs[0].is_type_attributes_form());
        assert!(form.defs[0].is_attributes());
        assert_eq!(
            form.defs[2].to_string(),
            "(def unwrap (attrs inline))".to_string()
        );
        assert!(form.defs[2].is_value_attributes_form());
        assert!(form.defs[2].is_attributes());
        assert_eq!(
            form.defs[4].to_string(),
            "(def unwrap (fun res (case res (match T id) (match E (panic E)))))".to_string()
        );
        assert!(form.defs[4].is_function_form());
        assert!(form.defs[4].is_value());
        assert_eq!(
            form.defs[8].to_string(),
            "(def res (unwrap (prod String StringError) \"res\"))".to_string()
        );
        assert!(form.defs[8].is_application_form());
        assert!(form.defs[8].is_value());
        assert_eq!(
            form.value.to_string(),
            "(return (prod res res2))".to_string()
        );
    }
}
