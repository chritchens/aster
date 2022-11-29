use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppFormValue;
use crate::form::attrs_form::AttrsForm;
use crate::form::export_form::ExportForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::import_form::ImportForm;
use crate::form::sig_form::SigForm;
use crate::form::type_form::TypeForm;
use crate::form::val_form::ValForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BlockFormEntry {
    Empty(SimpleValue),
    ImportForm(Box<ImportForm>),
    ExportForm(Box<ExportForm>),
    AttrsForm(Box<AttrsForm>),
    TypeForm(Box<TypeForm>),
    SigForm(Box<SigForm>),
    ValForm(Box<ValForm>),
}

impl Default for BlockFormEntry {
    fn default() -> BlockFormEntry {
        BlockFormEntry::Empty(SimpleValue::default())
    }
}

impl BlockFormEntry {
    pub fn file(&self) -> String {
        match self {
            BlockFormEntry::Empty(empty) => empty.file(),
            BlockFormEntry::ImportForm(form) => form.file(),
            BlockFormEntry::ExportForm(form) => form.file(),
            BlockFormEntry::AttrsForm(form) => form.file(),
            BlockFormEntry::TypeForm(form) => form.file(),
            BlockFormEntry::SigForm(form) => form.file(),
            BlockFormEntry::ValForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            BlockFormEntry::Empty(empty) => empty.loc(),
            BlockFormEntry::ImportForm(form) => form.loc(),
            BlockFormEntry::ExportForm(form) => form.loc(),
            BlockFormEntry::AttrsForm(form) => form.loc(),
            BlockFormEntry::TypeForm(form) => form.loc(),
            BlockFormEntry::SigForm(form) => form.loc(),
            BlockFormEntry::ValForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            BlockFormEntry::Empty(_) => "()".into(),
            BlockFormEntry::ImportForm(form) => form.to_string(),
            BlockFormEntry::ExportForm(form) => form.to_string(),
            BlockFormEntry::AttrsForm(form) => form.to_string(),
            BlockFormEntry::TypeForm(form) => form.to_string(),
            BlockFormEntry::SigForm(form) => form.to_string(),
            BlockFormEntry::ValForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for BlockFormEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub type BlockFormValue = AppFormValue;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct BlockForm {
    pub tokens: Box<Tokens>,
    pub entries: Vec<BlockFormEntry>,
}

impl BlockForm {
    pub fn new() -> BlockForm {
        BlockForm::default()
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
            BlockFormEntry::ImportForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_export(&self, idx: usize) -> Option<Box<ExportForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            BlockFormEntry::ExportForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_type(&self, idx: usize) -> Option<Box<TypeForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            BlockFormEntry::TypeForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_signature(&self, idx: usize) -> Option<Box<SigForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            BlockFormEntry::SigForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_attributes(&self, idx: usize) -> Option<Box<AttrsForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            BlockFormEntry::AttrsForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_definition(&self, idx: usize) -> Option<Box<ValForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            BlockFormEntry::ValForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entries_to_string(&self) -> String {
        let len = self.entries.len();

        match len {
            1 => self.entries[0].to_string(),
            _ => self
                .entries
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for entry in self.entries.iter() {
            match entry.clone() {
                BlockFormEntry::ImportForm(form) => {
                    params.extend(form.all_parameters());
                }
                BlockFormEntry::ExportForm(form) => {
                    params.extend(form.all_parameters());
                }
                BlockFormEntry::AttrsForm(form) => {
                    params.extend(form.all_parameters());
                }
                BlockFormEntry::TypeForm(form) => {
                    params.extend(form.all_parameters());
                }
                BlockFormEntry::SigForm(form) => {
                    params.extend(form.all_parameters());
                }
                BlockFormEntry::ValForm(form) => {
                    params.push(form.name.clone());
                    params.extend(form.all_parameters());
                }
                _ => {}
            }
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for entry in self.entries.iter() {
            match entry.clone() {
                BlockFormEntry::ImportForm(form) => {
                    vars.extend(form.all_variables());
                }
                BlockFormEntry::ExportForm(form) => {
                    vars.extend(form.all_variables());
                }
                BlockFormEntry::AttrsForm(form) => {
                    vars.extend(form.all_variables());
                }
                BlockFormEntry::TypeForm(form) => {
                    vars.extend(form.all_variables());
                }
                BlockFormEntry::SigForm(form) => {
                    vars.extend(form.all_variables());
                }
                BlockFormEntry::ValForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<BlockForm> {
        if form.head.to_string() != "block" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a block keyword".into(),
            }));
        }

        if form.tail.is_empty() {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a value".into(),
            }));
        }

        let mut block_form = BlockForm::new();
        block_form.tokens = form.tokens.clone();

        for param in form.tail.iter().clone() {
            match param {
                FormTailElement::Form(form) => {
                    if let Ok(form) = ImportForm::from_form(&form) {
                        block_form
                            .entries
                            .push(BlockFormEntry::ImportForm(Box::new(form)));
                    } else if let Ok(form) = ExportForm::from_form(&form) {
                        block_form
                            .entries
                            .push(BlockFormEntry::ExportForm(Box::new(form)));
                    } else if let Ok(form) = AttrsForm::from_form(&form) {
                        block_form
                            .entries
                            .push(BlockFormEntry::AttrsForm(Box::new(form)));
                    } else if let Ok(form) = TypeForm::from_form(&form) {
                        block_form
                            .entries
                            .push(BlockFormEntry::TypeForm(Box::new(form)));
                    } else if let Ok(form) = SigForm::from_form(&form) {
                        block_form
                            .entries
                            .push(BlockFormEntry::SigForm(Box::new(form)));
                    } else if let Ok(form) = ValForm::from_form(&form) {
                        block_form
                            .entries
                            .push(BlockFormEntry::ValForm(Box::new(form)));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
                _ => {
                    println!("{}", form.to_string());
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a form".into(),
                    }));
                }
            }
        }

        Ok(block_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<BlockForm> {
        let form = Form::from_tokens(tokens)?;

        BlockForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<BlockForm> {
        let tokens = Tokens::from_str(s)?;

        BlockForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(block {})", self.entries_to_string())
    }
}

impl fmt::Display for BlockForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for BlockForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn block_form_from_str() {
        use super::BlockForm;

        let mut s = "(block (val e10 (math.exp (prod math.e 10))))";

        let mut res = BlockForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.entries_to_string(),
            "(val e10 (math.exp (prod math.e 10)))".to_string()
        );
        assert_eq!(
            form.to_string(),
            "(block (val e10 (math.exp (prod math.e 10))))".to_string()
        );

        s = "
        (block
            (import res () Result)

            (attrs Result union)
            (type Result (Sum T E))

            (attrs unwrap inline)
            (sig unwrap (Fun (Result T E) T))
            (val unwrap (fun res (case res (match T id) (match E panic))))

            (type StringError String)
            (type StringResult (Result String StringResult))

            (sig res String)
            (val res (unwrap \"res\"))

            (sig x StringError)
            (val x \"res2\")

            (sig res2 String)
            (val res2 (unwrap x)))";

        res = BlockForm::from_str(s);

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
            "(val unwrap (fun res (case res (match T id) (match E panic))))".to_string()
        );
        assert!(form.entry_as_definition(5).unwrap().is_function_form());
        assert!(form.entry_as_definition(5).unwrap().is_value());
        assert_eq!(
            form.entries[9].to_string(),
            "(val res (unwrap \"res\"))".to_string()
        );
        assert!(form.entry_as_definition(9).unwrap().is_application_form());
        assert!(form.entry_as_definition(9).unwrap().is_value());
    }
}
