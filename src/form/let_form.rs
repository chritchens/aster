use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::attrs_form::AttrsForm;
use crate::form::def_form::DefForm;
use crate::form::form::{Form, FormParam};
use crate::form::import_form::ImportForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LetFormEntry {
    Empty,
    ImportForm(Box<ImportForm>),
    AttrsForm(Box<AttrsForm>),
    DefForm(Box<DefForm>),
}

impl Default for LetFormEntry {
    fn default() -> LetFormEntry {
        LetFormEntry::Empty
    }
}

impl LetFormEntry {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            LetFormEntry::Empty => "()".into(),
            LetFormEntry::ImportForm(form) => form.to_string(),
            LetFormEntry::AttrsForm(form) => form.to_string(),
            LetFormEntry::DefForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for LetFormEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct LetForm {
    pub tokens: Box<Tokens>,
    pub entries: Vec<LetFormEntry>,
    pub value: AppForm,
}

impl LetForm {
    pub fn new() -> LetForm {
        LetForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn entry_as_import(&self, idx: usize) -> Option<Box<ImportForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            LetFormEntry::ImportForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_attributes(&self, idx: usize) -> Option<Box<AttrsForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            LetFormEntry::AttrsForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_definition(&self, idx: usize) -> Option<Box<DefForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            LetFormEntry::DefForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entries_to_string(&self) -> String {
        let len = self.entries.len();

        match len {
            1 => self.entries[0].to_string(),
            _ => format!(
                "(prod {})",
                self.entries
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
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
        let_form.tokens = form.tokens.clone();

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
                        if let Ok(form) = ImportForm::from_form(&form) {
                            let_form
                                .entries
                                .push(LetFormEntry::ImportForm(Box::new(form)));
                        } else if let Ok(form) = AttrsForm::from_form(&form) {
                            let_form
                                .entries
                                .push(LetFormEntry::AttrsForm(Box::new(form)));
                        } else if let Ok(form) = DefForm::from_form(&form) {
                            let_form.entries.push(LetFormEntry::DefForm(Box::new(form)));
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
        if self.entries.is_empty() {
            format!("(let {})", self.value.to_string(),)
        } else {
            format!(
                "(let {} {})",
                self.entries_to_string(),
                self.value.to_string(),
            )
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

        assert!(form.entries.is_empty());
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
            (import res () Result)

            (attrs Result union)
            (def Result (Sum T E))

            (attrs unwrap inline)
            (def unwrap (Fun (Result T E) T))
            (def unwrap (fun res (case res (match T id) (match E panic))))

            (def StringError String)
            (def StringResult (Result String StringResult))

            (def res String)
            (def res (unwrap \"res\"))
            (def x StringError)
            (def x \"res2\")
            (def res2 String)
            (def res2 (unwrap x)) # will panic

            # return as a synonym of `id`
            (return (prod res res2)))";

        res = LetForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.entries.len(), 14);
        assert_eq!(
            form.entries[0].to_string(),
            "(import res () Result)".to_string()
        );
        assert!(form.entry_as_import(0).is_some());
        assert_eq!(
            form.entries[1].to_string(),
            "(attrs Result union)".to_string()
        );
        assert!(form.entry_as_attributes(1).is_some());
        assert!(form.entry_as_attributes(1).unwrap().is_type_attributes());
        assert_eq!(
            form.entries[3].to_string(),
            "(attrs unwrap inline)".to_string()
        );
        assert!(form.entry_as_attributes(3).is_some());
        assert!(form.entry_as_attributes(3).unwrap().is_value_attributes());
        assert_eq!(
            form.entries[5].to_string(),
            "(def unwrap (fun res (case res (match T id) (match E panic))))".to_string()
        );
        assert!(form.entry_as_definition(5).unwrap().is_function_form());
        assert!(form.entry_as_definition(5).unwrap().is_value());
        assert_eq!(
            form.entries[9].to_string(),
            "(def res (unwrap \"res\"))".to_string()
        );
        assert!(form.entry_as_definition(9).unwrap().is_application_form());
        assert!(form.entry_as_definition(9).unwrap().is_value());
        assert_eq!(
            form.value.to_string(),
            "(return (prod res res2))".to_string()
        );
    }
}
