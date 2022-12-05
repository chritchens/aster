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
use crate::form::PairForm;
use crate::form::SigForm;
use crate::form::TypeForm;
use crate::form::ValForm;
use crate::form::VecForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::types::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum FormValue {
    ModuleForm(Box<ModuleForm>),
    BlockForm(Box<BlockForm>),
    ImportForm(Box<ImportForm>),
    ExportForm(Box<ExportForm>),
    AttrsForm(Box<AttrsForm>),
    TypeForm(Box<TypeForm>),
    SigForm(Box<SigForm>),
    ValForm(Box<ValForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    CaseForm(Box<CaseForm>),
    AppForm(Box<AppForm>),
    MapForm(Box<MapForm>),
    VecForm(Box<VecForm>),
    ArrForm(Box<ArrForm>),
    ListForm(Box<ListForm>),
    PairForm(Box<PairForm>),
    Type(Box<Type>),
}

impl Default for FormValue {
    fn default() -> FormValue {
        FormValue::ModuleForm(Box::new(ModuleForm::new()))
    }
}

impl FormValue {
    pub fn new() -> FormValue {
        FormValue::default()
    }

    pub fn file(&self) -> String {
        match self {
            FormValue::ModuleForm(form) => form.file(),
            FormValue::BlockForm(form) => form.file(),
            FormValue::ImportForm(form) => form.file(),
            FormValue::ExportForm(form) => form.file(),
            FormValue::AttrsForm(form) => form.file(),
            FormValue::TypeForm(form) => form.file(),
            FormValue::SigForm(form) => form.file(),
            FormValue::ValForm(form) => form.file(),
            FormValue::FunForm(form) => form.file(),
            FormValue::LetForm(form) => form.file(),
            FormValue::CaseForm(form) => form.file(),
            FormValue::AppForm(form) => form.file(),
            FormValue::MapForm(form) => form.file(),
            FormValue::VecForm(form) => form.file(),
            FormValue::ArrForm(form) => form.file(),
            FormValue::ListForm(form) => form.file(),
            FormValue::PairForm(form) => form.file(),
            FormValue::Type(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            FormValue::ModuleForm(form) => form.loc(),
            FormValue::BlockForm(form) => form.loc(),
            FormValue::ImportForm(form) => form.loc(),
            FormValue::ExportForm(form) => form.loc(),
            FormValue::AttrsForm(form) => form.loc(),
            FormValue::TypeForm(form) => form.loc(),
            FormValue::SigForm(form) => form.loc(),
            FormValue::ValForm(form) => form.loc(),
            FormValue::FunForm(form) => form.loc(),
            FormValue::LetForm(form) => form.loc(),
            FormValue::CaseForm(form) => form.loc(),
            FormValue::AppForm(form) => form.loc(),
            FormValue::MapForm(form) => form.loc(),
            FormValue::VecForm(form) => form.loc(),
            FormValue::ArrForm(form) => form.loc(),
            FormValue::ListForm(form) => form.loc(),
            FormValue::PairForm(form) => form.loc(),
            FormValue::Type(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FormValue::ModuleForm(form) => form.to_string(),
            FormValue::BlockForm(form) => form.to_string(),
            FormValue::ImportForm(form) => form.to_string(),
            FormValue::ExportForm(form) => form.to_string(),
            FormValue::AttrsForm(form) => form.to_string(),
            FormValue::TypeForm(form) => form.to_string(),
            FormValue::SigForm(form) => form.to_string(),
            FormValue::ValForm(form) => form.to_string(),
            FormValue::FunForm(form) => form.to_string(),
            FormValue::LetForm(form) => form.to_string(),
            FormValue::CaseForm(form) => form.to_string(),
            FormValue::AppForm(form) => form.to_string(),
            FormValue::MapForm(form) => form.to_string(),
            FormValue::VecForm(form) => form.to_string(),
            FormValue::ArrForm(form) => form.to_string(),
            FormValue::ListForm(form) => form.to_string(),
            FormValue::PairForm(form) => form.to_string(),
            FormValue::Type(form) => form.to_string(),
        }
    }

    #[allow(clippy::should_implement_trait)]
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
            FormValue::ModuleForm(Box::new(form))
        } else if let Ok(form) = BlockForm::from_form(form) {
            FormValue::BlockForm(Box::new(form))
        } else if let Ok(form) = ImportForm::from_form(form) {
            FormValue::ImportForm(Box::new(form))
        } else if let Ok(form) = ExportForm::from_form(form) {
            FormValue::ExportForm(Box::new(form))
        } else if let Ok(form) = AttrsForm::from_form(form) {
            FormValue::AttrsForm(Box::new(form))
        } else if let Ok(form) = TypeForm::from_form(form) {
            FormValue::TypeForm(Box::new(form))
        } else if let Ok(form) = SigForm::from_form(form) {
            FormValue::SigForm(Box::new(form))
        } else if let Ok(form) = ValForm::from_form(form) {
            FormValue::ValForm(Box::new(form))
        } else if let Ok(form) = FunForm::from_form(form) {
            FormValue::FunForm(Box::new(form))
        } else if let Ok(form) = LetForm::from_form(form) {
            FormValue::LetForm(Box::new(form))
        } else if let Ok(form) = CaseForm::from_form(form) {
            FormValue::CaseForm(Box::new(form))
        } else if let Ok(form) = AppForm::from_form(form) {
            FormValue::AppForm(Box::new(form))
        } else if let Ok(form) = MapForm::from_form(form) {
            FormValue::MapForm(Box::new(form))
        } else if let Ok(form) = VecForm::from_form(form) {
            FormValue::VecForm(Box::new(form))
        } else if let Ok(form) = ArrForm::from_form(form) {
            FormValue::ArrForm(Box::new(form))
        } else if let Ok(form) = ListForm::from_form(form) {
            FormValue::ListForm(Box::new(form))
        } else if let Ok(form) = PairForm::from_form(form) {
            FormValue::PairForm(Box::new(form))
        } else if let Ok(form) = Type::from_form(form) {
            FormValue::Type(Box::new(form))
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

impl std::str::FromStr for FormValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}
