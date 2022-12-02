use crate::error::{Error, SyntacticError};
use crate::form::AppForm;
use crate::form::ArrForm;
use crate::form::AttrsForm;
use crate::form::BlockForm;
use crate::form::CaseForm;
use crate::form::ExportForm;
use crate::form::Form;
use crate::form::FunForm;
use crate::form::ImportForm;
use crate::form::LetForm;
use crate::form::ListForm;
use crate::form::MapForm;
use crate::form::ModuleForm;
use crate::form::ProdForm;
use crate::form::SigForm;
use crate::form::TypeForm;
use crate::form::TypesForm;
use crate::form::ValForm;
use crate::form::VecForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FormValue {
    Module(Box<ModuleForm>),
    Block(Box<BlockForm>),
    Import(Box<ImportForm>),
    Export(Box<ExportForm>),
    Attrs(Box<AttrsForm>),
    Type(Box<TypeForm>),
    Sig(Box<SigForm>),
    Val(Box<ValForm>),
    Fun(Box<FunForm>),
    Let(Box<LetForm>),
    Case(Box<CaseForm>),
    App(Box<AppForm>),
    Map(Box<MapForm>),
    Vec(Box<VecForm>),
    Arr(Box<ArrForm>),
    List(Box<ListForm>),
    Prod(Box<ProdForm>),
    Types(Box<TypesForm>),
}

impl Default for FormValue {
    fn default() -> FormValue {
        FormValue::Module(Box::new(ModuleForm::new()))
    }
}

impl FormValue {
    pub fn new() -> FormValue {
        FormValue::default()
    }

    pub fn file(&self) -> String {
        match self {
            FormValue::Module(form) => form.file(),
            FormValue::Block(form) => form.file(),
            FormValue::Import(form) => form.file(),
            FormValue::Export(form) => form.file(),
            FormValue::Attrs(form) => form.file(),
            FormValue::Type(form) => form.file(),
            FormValue::Sig(form) => form.file(),
            FormValue::Val(form) => form.file(),
            FormValue::Fun(form) => form.file(),
            FormValue::Let(form) => form.file(),
            FormValue::Case(form) => form.file(),
            FormValue::App(form) => form.file(),
            FormValue::Map(form) => form.file(),
            FormValue::Vec(form) => form.file(),
            FormValue::Arr(form) => form.file(),
            FormValue::List(form) => form.file(),
            FormValue::Prod(form) => form.file(),
            FormValue::Types(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            FormValue::Module(form) => form.loc(),
            FormValue::Block(form) => form.loc(),
            FormValue::Import(form) => form.loc(),
            FormValue::Export(form) => form.loc(),
            FormValue::Attrs(form) => form.loc(),
            FormValue::Type(form) => form.loc(),
            FormValue::Sig(form) => form.loc(),
            FormValue::Val(form) => form.loc(),
            FormValue::Fun(form) => form.loc(),
            FormValue::Let(form) => form.loc(),
            FormValue::Case(form) => form.loc(),
            FormValue::App(form) => form.loc(),
            FormValue::Map(form) => form.loc(),
            FormValue::Vec(form) => form.loc(),
            FormValue::Arr(form) => form.loc(),
            FormValue::List(form) => form.loc(),
            FormValue::Prod(form) => form.loc(),
            FormValue::Types(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FormValue::Module(form) => form.to_string(),
            FormValue::Block(form) => form.to_string(),
            FormValue::Import(form) => form.to_string(),
            FormValue::Export(form) => form.to_string(),
            FormValue::Attrs(form) => form.to_string(),
            FormValue::Type(form) => form.to_string(),
            FormValue::Sig(form) => form.to_string(),
            FormValue::Val(form) => form.to_string(),
            FormValue::Fun(form) => form.to_string(),
            FormValue::Let(form) => form.to_string(),
            FormValue::Case(form) => form.to_string(),
            FormValue::App(form) => form.to_string(),
            FormValue::Map(form) => form.to_string(),
            FormValue::Vec(form) => form.to_string(),
            FormValue::Arr(form) => form.to_string(),
            FormValue::List(form) => form.to_string(),
            FormValue::Prod(form) => form.to_string(),
            FormValue::Types(form) => form.to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<FormValue> {
        let tokens = Tokens::from_str(s)?;

        FormValue::from_tokens(&tokens)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<FormValue> {
        let form = Form::from_tokens(tokens)?;

        FormValue::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<FormValue> {
        let form_value = if let Ok(form) = ModuleForm::from_form(form) {
            FormValue::Module(Box::new(form))
        } else if let Ok(form) = BlockForm::from_form(form) {
            FormValue::Block(Box::new(form))
        } else if let Ok(form) = ImportForm::from_form(form) {
            FormValue::Import(Box::new(form))
        } else if let Ok(form) = ExportForm::from_form(form) {
            FormValue::Export(Box::new(form))
        } else if let Ok(form) = AttrsForm::from_form(form) {
            FormValue::Attrs(Box::new(form))
        } else if let Ok(form) = TypeForm::from_form(form) {
            FormValue::Type(Box::new(form))
        } else if let Ok(form) = SigForm::from_form(form) {
            FormValue::Sig(Box::new(form))
        } else if let Ok(form) = ValForm::from_form(form) {
            FormValue::Val(Box::new(form))
        } else if let Ok(form) = FunForm::from_form(form) {
            FormValue::Fun(Box::new(form))
        } else if let Ok(form) = LetForm::from_form(form) {
            FormValue::Let(Box::new(form))
        } else if let Ok(form) = CaseForm::from_form(form) {
            FormValue::Case(Box::new(form))
        } else if let Ok(form) = AppForm::from_form(form) {
            FormValue::App(Box::new(form))
        } else if let Ok(form) = MapForm::from_form(form) {
            FormValue::Map(Box::new(form))
        } else if let Ok(form) = VecForm::from_form(form) {
            FormValue::Vec(Box::new(form))
        } else if let Ok(form) = ArrForm::from_form(form) {
            FormValue::Arr(Box::new(form))
        } else if let Ok(form) = ListForm::from_form(form) {
            FormValue::List(Box::new(form))
        } else if let Ok(form) = ProdForm::from_form(form) {
            FormValue::Prod(Box::new(form))
        } else if let Ok(form) = TypesForm::from_form(form) {
            FormValue::Types(Box::new(form))
        } else {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "unknown form".into(),
            }));
        };

        Ok(form_value)
    }
}

impl fmt::Display for FormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
