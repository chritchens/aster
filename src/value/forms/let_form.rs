use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::app_form::AppForm;
use crate::value::forms::app_form::AppFormValue;
use crate::value::forms::attrs_form::AttrsForm;
use crate::value::forms::case_form::CaseForm;
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::forms::fun_form::FunForm;
use crate::value::forms::import_form::ImportForm;
use crate::value::forms::pair_form::PairForm;
use crate::value::forms::sig_form::SigForm;
use crate::value::forms::type_form::TypeForm;
use crate::value::forms::val_form::ValForm;
use crate::value::SimpleValue;
use crate::value::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum LetFormEntry {
    Empty(SimpleValue),
    ImportForm(Box<ImportForm>),
    AttrsForm(Box<AttrsForm>),
    TypeForm(Box<TypeForm>),
    SigForm(Box<SigForm>),
    ValForm(Box<ValForm>),
}

impl Default for LetFormEntry {
    fn default() -> LetFormEntry {
        LetFormEntry::Empty(SimpleValue::default())
    }
}

impl LetFormEntry {
    pub fn file(&self) -> String {
        match self {
            LetFormEntry::Empty(empty) => empty.file(),
            LetFormEntry::ImportForm(form) => form.file(),
            LetFormEntry::AttrsForm(form) => form.file(),
            LetFormEntry::TypeForm(form) => form.file(),
            LetFormEntry::SigForm(form) => form.file(),
            LetFormEntry::ValForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            LetFormEntry::Empty(empty) => empty.loc(),
            LetFormEntry::ImportForm(form) => form.loc(),
            LetFormEntry::AttrsForm(form) => form.loc(),
            LetFormEntry::TypeForm(form) => form.loc(),
            LetFormEntry::SigForm(form) => form.loc(),
            LetFormEntry::ValForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            LetFormEntry::Empty(_) => "()".into(),
            LetFormEntry::ImportForm(form) => form.to_string(),
            LetFormEntry::AttrsForm(form) => form.to_string(),
            LetFormEntry::TypeForm(form) => form.to_string(),
            LetFormEntry::SigForm(form) => form.to_string(),
            LetFormEntry::ValForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for LetFormEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub type LetFormValue = AppFormValue;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct LetForm {
    pub tokens: Box<Tokens>,
    pub entries: Vec<LetFormEntry>,
    pub value: LetFormValue,
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

    pub fn entry_as_type(&self, idx: usize) -> Option<Box<TypeForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            LetFormEntry::TypeForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_signature(&self, idx: usize) -> Option<Box<SigForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            LetFormEntry::SigForm(form) => Some(form),
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

    pub fn entry_as_definition(&self, idx: usize) -> Option<Box<ValForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            LetFormEntry::ValForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entries_to_string(&self) -> String {
        self.entries
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for entry in self.entries.iter() {
            match entry.clone() {
                LetFormEntry::ImportForm(form) => {
                    params.extend(form.all_parameters());
                }
                LetFormEntry::ValForm(form) => {
                    params.push(form.name.clone());
                    params.extend(form.all_parameters());
                }
                _ => {}
            }
        }

        params.extend(self.value.all_parameters());

        params
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        for entry in self.entries.iter() {
            match entry.clone() {
                LetFormEntry::AttrsForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                LetFormEntry::ValForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                _ => {}
            }
        }

        value_vars.extend(self.value.all_value_variables());

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        for entry in self.entries.iter() {
            match entry.clone() {
                LetFormEntry::AttrsForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                LetFormEntry::TypeForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                LetFormEntry::SigForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                LetFormEntry::ValForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                _ => {}
            }
        }

        type_vars.extend(self.value.all_type_variables());

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for entry in self.entries.iter() {
            match entry.clone() {
                LetFormEntry::ImportForm(form) => {
                    vars.extend(form.all_variables());
                }
                LetFormEntry::AttrsForm(form) => {
                    vars.extend(form.all_variables());
                }
                LetFormEntry::TypeForm(form) => {
                    vars.extend(form.all_variables());
                }
                LetFormEntry::SigForm(form) => {
                    vars.extend(form.all_variables());
                }
                LetFormEntry::ValForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars.extend(self.value.all_variables());

        vars
    }

    pub fn from_form(form: &Form) -> Result<LetForm> {
        if form.head.to_string() != "let" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a let keyword".into(),
            }));
        }

        let len = form.tail.len();

        if len == 0 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a value".into(),
            }));
        }

        let mut let_form = LetForm::new();
        let_form.tokens = form.tokens.clone();

        if len == 1 {
            match form.tail[0].clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        let_form.value = LetFormValue::Ignore(value);
                    }
                    SimpleValue::Empty(_) => {
                        let_form.value = LetFormValue::Empty(value);
                    }
                    SimpleValue::Panic(_) => {
                        let_form.value = LetFormValue::Panic(value);
                    }
                    SimpleValue::Atomic(_) => {
                        let_form.value = LetFormValue::Atomic(value);
                    }
                    SimpleValue::ValueSymbol(_) => {
                        let_form.value = LetFormValue::ValueSymbol(value);
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        let_form.value = LetFormValue::ValuePathSymbol(value);
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unexpected value".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = PairForm::from_form(&form) {
                        let_form.value = LetFormValue::PairForm(Box::new(form));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        let_form.value = LetFormValue::FunForm(Box::new(form));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        let_form.value = LetFormValue::LetForm(Box::new(form));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        let_form.value = LetFormValue::CaseForm(Box::new(form));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        let_form.value = LetFormValue::AppForm(Box::new(form));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        if len > 1 {
            for param in form.tail[0..(len - 1)].iter().clone() {
                match param {
                    FormTailElement::Form(form) => {
                        if let Ok(form) = ImportForm::from_form(form) {
                            let_form
                                .entries
                                .push(LetFormEntry::ImportForm(Box::new(form)));
                        } else if let Ok(form) = AttrsForm::from_form(form) {
                            let_form
                                .entries
                                .push(LetFormEntry::AttrsForm(Box::new(form)));
                        } else if let Ok(form) = TypeForm::from_form(form) {
                            let_form
                                .entries
                                .push(LetFormEntry::TypeForm(Box::new(form)));
                        } else if let Ok(form) = SigForm::from_form(form) {
                            let_form.entries.push(LetFormEntry::SigForm(Box::new(form)));
                        } else if let Ok(form) = ValForm::from_form(form) {
                            let_form.entries.push(LetFormEntry::ValForm(Box::new(form)));
                        } else {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "unexpected form".into(),
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

            match form.tail[len - 1].clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        let_form.value = LetFormValue::Ignore(value);
                    }
                    SimpleValue::Empty(_) => {
                        let_form.value = LetFormValue::Empty(value);
                    }
                    SimpleValue::Panic(_) => {
                        let_form.value = LetFormValue::Panic(value);
                    }
                    SimpleValue::Atomic(_) => {
                        let_form.value = LetFormValue::Atomic(value);
                    }
                    SimpleValue::ValueSymbol(_) => {
                        let_form.value = LetFormValue::ValueSymbol(value);
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        let_form.value = LetFormValue::ValuePathSymbol(value);
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unexpected value".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = PairForm::from_form(&form) {
                        let_form.value = LetFormValue::PairForm(Box::new(form));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        let_form.value = LetFormValue::FunForm(Box::new(form));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        let_form.value = LetFormValue::LetForm(Box::new(form));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        let_form.value = LetFormValue::CaseForm(Box::new(form));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        let_form.value = LetFormValue::AppForm(Box::new(form));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        Ok(let_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<LetForm> {
        let form = Form::from_tokens(tokens)?;

        LetForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
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

impl std::str::FromStr for LetForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn let_form_from_str() {
        use super::LetForm;

        let mut s = "(let (math.exp math.e 10))";

        let mut res = LetForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert!(form.entries.is_empty());
        assert_eq!(form.value.to_string(), "(math.exp math.e 10)".to_string());
        assert_eq!(form.to_string(), "(let (math.exp math.e 10))".to_string());

        s = "
        (let
            (type StringError String)
            (import result (list String StringError) (list Result unwrap))

            (sig res String)
            (val res (unwrap \"res\"))
            (sig x StringError)
            (val x \"res2\")
            (sig res2 String)
            (val res2 (unwrap x)) # will panic

            # return as a synonym of `id`
            (return (pair res res2)))";

        res = LetForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.entries.len(), 8);
        assert_eq!(
            form.entries[1].to_string(),
            "(import result (list String StringError) (list Result unwrap))".to_string()
        );
        assert!(form.entry_as_import(1).is_some());
        assert_eq!(form.entries[2].to_string(), "(sig res String)".to_string());
        assert!(form.entry_as_signature(2).is_some());
        assert!(form.entry_as_signature(2).unwrap().is_type_keyword());
        assert_eq!(
            form.value.to_string(),
            "(return (pair res res2))".to_string()
        );
    }
}
