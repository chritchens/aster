use crate::error::{Error, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::arr_form::ArrForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::list_form::ListForm;
use crate::form::map_form::MapForm;
use crate::form::pair_form::PairForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::types::Type;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum VecFormValue {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueKeyword(SimpleValue),
    TypeKeyword(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    Type(Box<Type>),
    FunForm(Box<FunForm>),
    CaseForm(Box<CaseForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    PairForm(Box<PairForm>),
    ListForm(Box<ListForm>),
    ArrForm(Box<ArrForm>),
    MapForm(Box<MapForm>),
    VecForm(Box<VecForm>),
}

impl Default for VecFormValue {
    fn default() -> VecFormValue {
        VecFormValue::Empty(SimpleValue::new())
    }
}

impl VecFormValue {
    pub fn file(&self) -> String {
        match self {
            VecFormValue::Ignore(ignore) => ignore.file(),
            VecFormValue::Empty(empty) => empty.file(),
            VecFormValue::Panic(panic) => panic.file(),
            VecFormValue::Atomic(atomic) => atomic.file(),
            VecFormValue::ValueKeyword(keyword) => keyword.file(),
            VecFormValue::TypeKeyword(keyword) => keyword.file(),
            VecFormValue::ValueSymbol(symbol) => symbol.file(),
            VecFormValue::TypeSymbol(symbol) => symbol.file(),
            VecFormValue::ValuePathSymbol(symbol) => symbol.file(),
            VecFormValue::TypePathSymbol(symbol) => symbol.file(),
            VecFormValue::Type(form) => form.file(),
            VecFormValue::FunForm(form) => form.file(),
            VecFormValue::CaseForm(form) => form.file(),
            VecFormValue::LetForm(form) => form.file(),
            VecFormValue::AppForm(form) => form.file(),
            VecFormValue::PairForm(form) => form.file(),
            VecFormValue::ListForm(form) => form.file(),
            VecFormValue::ArrForm(form) => form.file(),
            VecFormValue::MapForm(form) => form.file(),
            VecFormValue::VecForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            VecFormValue::Ignore(ignore) => ignore.loc(),
            VecFormValue::Empty(empty) => empty.loc(),
            VecFormValue::Panic(panic) => panic.loc(),
            VecFormValue::Atomic(atomic) => atomic.loc(),
            VecFormValue::ValueKeyword(keyword) => keyword.loc(),
            VecFormValue::TypeKeyword(keyword) => keyword.loc(),
            VecFormValue::ValueSymbol(symbol) => symbol.loc(),
            VecFormValue::TypeSymbol(symbol) => symbol.loc(),
            VecFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            VecFormValue::TypePathSymbol(symbol) => symbol.loc(),
            VecFormValue::Type(form) => form.loc(),
            VecFormValue::FunForm(form) => form.loc(),
            VecFormValue::CaseForm(form) => form.loc(),
            VecFormValue::LetForm(form) => form.loc(),
            VecFormValue::AppForm(form) => form.loc(),
            VecFormValue::PairForm(form) => form.loc(),
            VecFormValue::ListForm(form) => form.loc(),
            VecFormValue::ArrForm(form) => form.loc(),
            VecFormValue::MapForm(form) => form.loc(),
            VecFormValue::VecForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            VecFormValue::Ignore(_) => "_".into(),
            VecFormValue::Empty(_) => "()".into(),
            VecFormValue::Panic(_) => "panic".into(),
            VecFormValue::Atomic(atomic) => atomic.to_string(),
            VecFormValue::ValueKeyword(keyword) => keyword.to_string(),
            VecFormValue::TypeKeyword(keyword) => keyword.to_string(),
            VecFormValue::ValueSymbol(symbol) => symbol.to_string(),
            VecFormValue::TypeSymbol(symbol) => symbol.to_string(),
            VecFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            VecFormValue::TypePathSymbol(symbol) => symbol.to_string(),
            VecFormValue::Type(form) => form.to_string(),
            VecFormValue::FunForm(form) => form.to_string(),
            VecFormValue::CaseForm(form) => form.to_string(),
            VecFormValue::LetForm(form) => form.to_string(),
            VecFormValue::AppForm(form) => form.to_string(),
            VecFormValue::PairForm(form) => form.to_string(),
            VecFormValue::ListForm(form) => form.to_string(),
            VecFormValue::ArrForm(form) => form.to_string(),
            VecFormValue::MapForm(form) => form.to_string(),
            VecFormValue::VecForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for VecFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct VecForm {
    pub tokens: Box<Tokens>,
    pub values: Vec<VecFormValue>,
}

impl VecForm {
    pub fn new() -> VecForm {
        VecForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn can_be_parameter(&self) -> bool {
        for value in self.values.iter() {
            match value {
                VecFormValue::Ignore(_)
                | VecFormValue::Empty(_)
                | VecFormValue::Atomic(_)
                | VecFormValue::ValueKeyword(_)
                | VecFormValue::TypeKeyword(_)
                | VecFormValue::TypeSymbol(_)
                | VecFormValue::TypePathSymbol(_)
                | VecFormValue::FunForm(_)
                | VecFormValue::Type(_)
                | VecFormValue::CaseForm(_)
                | VecFormValue::LetForm(_)
                | VecFormValue::AppForm(_) => return false,
                VecFormValue::PairForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                VecFormValue::ListForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                VecFormValue::ArrForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                VecFormValue::MapForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                VecFormValue::VecForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn values_to_string(&self) -> String {
        self.values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for value in self.values.iter() {
            match value.clone() {
                VecFormValue::Type(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::FunForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::CaseForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::LetForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::AppForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::PairForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::ListForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::ArrForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::MapForm(form) => {
                    params.extend(form.all_parameters());
                }
                VecFormValue::VecForm(form) => {
                    params.extend(form.all_parameters());
                }
                _ => {}
            }
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for value in self.values.iter() {
            match value.clone() {
                VecFormValue::ValueSymbol(value) => {
                    vars.push(value);
                }
                VecFormValue::TypeSymbol(value) => {
                    vars.push(value);
                }
                VecFormValue::ValuePathSymbol(value) => {
                    vars.push(value);
                }
                VecFormValue::TypePathSymbol(value) => {
                    vars.push(value);
                }
                VecFormValue::Type(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::FunForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::CaseForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::LetForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::AppForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::PairForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::ListForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::ArrForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::MapForm(form) => {
                    vars.extend(form.all_variables());
                }
                VecFormValue::VecForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<VecForm> {
        if form.head.to_string() != "vec" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a vec keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least two values".into(),
            }));
        }

        let mut vec = VecForm::new();
        vec.tokens = form.tokens.clone();

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        vec.values.push(VecFormValue::Ignore(value));
                    }
                    SimpleValue::Empty(_) => {
                        vec.values.push(VecFormValue::Empty(value));
                    }
                    SimpleValue::Atomic(_) => {
                        vec.values.push(VecFormValue::Atomic(value));
                    }
                    SimpleValue::ValueKeyword(_) => {
                        vec.values.push(VecFormValue::ValueKeyword(value));
                    }
                    SimpleValue::TypeKeyword(_) => {
                        vec.values.push(VecFormValue::TypeKeyword(value));
                    }
                    SimpleValue::ValueSymbol(_) => {
                        vec.values.push(VecFormValue::ValueSymbol(value));
                    }
                    SimpleValue::TypeSymbol(_) => {
                        vec.values.push(VecFormValue::TypeSymbol(value));
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        vec.values.push(VecFormValue::ValuePathSymbol(value));
                    }
                    SimpleValue::TypePathSymbol(_) => {
                        vec.values.push(VecFormValue::TypePathSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unxexpected value".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = Type::from_form(&form) {
                        vec.values.push(VecFormValue::Type(Box::new(form)));
                    } else if let Ok(form) = PairForm::from_form(&form) {
                        vec.values.push(VecFormValue::PairForm(Box::new(form)));
                    } else if let Ok(form) = ListForm::from_form(&form) {
                        vec.values.push(VecFormValue::ListForm(Box::new(form)));
                    } else if let Ok(form) = ArrForm::from_form(&form) {
                        vec.values.push(VecFormValue::ArrForm(Box::new(form)));
                    } else if let Ok(form) = MapForm::from_form(&form) {
                        vec.values.push(VecFormValue::MapForm(Box::new(form)));
                    } else if let Ok(form) = VecForm::from_form(&form) {
                        vec.values.push(VecFormValue::VecForm(Box::new(form)));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        vec.values.push(VecFormValue::FunForm(Box::new(form)));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        vec.values.push(VecFormValue::CaseForm(Box::new(form)));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        vec.values.push(VecFormValue::LetForm(Box::new(form)));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        vec.values.push(VecFormValue::AppForm(Box::new(form)))
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        Ok(vec)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<VecForm> {
        let form = Form::from_tokens(tokens)?;

        VecForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<VecForm> {
        let tokens = Tokens::from_str(s)?;

        VecForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(vec {})", self.values_to_string())
    }
}

impl fmt::Display for VecForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for VecForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec_form_from_str() {
        use super::VecForm;

        let mut s = "(vec a A)";

        let mut res = VecForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["a".to_string(), "A".to_string()]
        );
        assert_eq!(form.values_to_string(), "a A".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(vec moduleX.X y)";

        res = VecForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["moduleX.X".to_string(), "y".to_string()]
        );
        assert_eq!(form.values_to_string(), "moduleX.X y".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(vec 0 (Fun A B))";

        res = VecForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["0".to_string(), "(Fun A B)".to_string()]
        );
        assert_eq!(form.values_to_string(), "0 (Fun A B)".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
