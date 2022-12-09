use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::app_form::AppForm;
use crate::value::forms::arr_form::ArrForm;
use crate::value::forms::case_form::CaseForm;
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::forms::fun_form::FunForm;
use crate::value::forms::let_form::LetForm;
use crate::value::forms::list_form::ListForm;
use crate::value::forms::map_form::MapForm;
use crate::value::forms::vec_form::VecForm;
use crate::value::types::Type;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum PairFormValue {
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
    MapForm(Box<MapForm>),
    VecForm(Box<VecForm>),
    ArrForm(Box<ArrForm>),
    ListForm(Box<ListForm>),
    PairForm(Box<PairForm>),
    FunForm(Box<FunForm>),
    CaseForm(Box<CaseForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
}

impl Default for PairFormValue {
    fn default() -> PairFormValue {
        PairFormValue::Empty(SimpleValue::new())
    }
}

impl PairFormValue {
    pub fn file(&self) -> String {
        match self {
            PairFormValue::Ignore(ignore) => ignore.file(),
            PairFormValue::Empty(empty) => empty.file(),
            PairFormValue::Panic(panic) => panic.file(),
            PairFormValue::Atomic(atomic) => atomic.file(),
            PairFormValue::ValueKeyword(keyword) => keyword.file(),
            PairFormValue::TypeKeyword(keyword) => keyword.file(),
            PairFormValue::ValueSymbol(symbol) => symbol.file(),
            PairFormValue::TypeSymbol(symbol) => symbol.file(),
            PairFormValue::ValuePathSymbol(symbol) => symbol.file(),
            PairFormValue::TypePathSymbol(symbol) => symbol.file(),
            PairFormValue::Type(form) => form.file(),
            PairFormValue::MapForm(form) => form.file(),
            PairFormValue::VecForm(form) => form.file(),
            PairFormValue::ArrForm(form) => form.file(),
            PairFormValue::ListForm(form) => form.file(),
            PairFormValue::PairForm(form) => form.file(),
            PairFormValue::FunForm(form) => form.file(),
            PairFormValue::CaseForm(form) => form.file(),
            PairFormValue::LetForm(form) => form.file(),
            PairFormValue::AppForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            PairFormValue::Ignore(ignore) => ignore.loc(),
            PairFormValue::Empty(empty) => empty.loc(),
            PairFormValue::Panic(panic) => panic.loc(),
            PairFormValue::Atomic(atomic) => atomic.loc(),
            PairFormValue::ValueKeyword(keyword) => keyword.loc(),
            PairFormValue::TypeKeyword(keyword) => keyword.loc(),
            PairFormValue::ValueSymbol(symbol) => symbol.loc(),
            PairFormValue::TypeSymbol(symbol) => symbol.loc(),
            PairFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            PairFormValue::TypePathSymbol(symbol) => symbol.loc(),
            PairFormValue::Type(form) => form.loc(),
            PairFormValue::MapForm(form) => form.loc(),
            PairFormValue::VecForm(form) => form.loc(),
            PairFormValue::ArrForm(form) => form.loc(),
            PairFormValue::ListForm(form) => form.loc(),
            PairFormValue::PairForm(form) => form.loc(),
            PairFormValue::FunForm(form) => form.loc(),
            PairFormValue::CaseForm(form) => form.loc(),
            PairFormValue::LetForm(form) => form.loc(),
            PairFormValue::AppForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            PairFormValue::Ignore(_) => "_".into(),
            PairFormValue::Empty(_) => "()".into(),
            PairFormValue::Panic(_) => "panic".into(),
            PairFormValue::Atomic(atomic) => atomic.to_string(),
            PairFormValue::ValueKeyword(keyword) => keyword.to_string(),
            PairFormValue::TypeKeyword(keyword) => keyword.to_string(),
            PairFormValue::ValueSymbol(symbol) => symbol.to_string(),
            PairFormValue::TypeSymbol(symbol) => symbol.to_string(),
            PairFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            PairFormValue::TypePathSymbol(symbol) => symbol.to_string(),
            PairFormValue::Type(form) => form.to_string(),
            PairFormValue::MapForm(form) => form.to_string(),
            PairFormValue::VecForm(form) => form.to_string(),
            PairFormValue::ArrForm(form) => form.to_string(),
            PairFormValue::ListForm(form) => form.to_string(),
            PairFormValue::PairForm(form) => form.to_string(),
            PairFormValue::FunForm(form) => form.to_string(),
            PairFormValue::CaseForm(form) => form.to_string(),
            PairFormValue::LetForm(form) => form.to_string(),
            PairFormValue::AppForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for PairFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct PairForm {
    pub tokens: Box<Tokens>,
    pub first: PairFormValue,
    pub second: PairFormValue,
}

impl PairForm {
    pub fn new() -> PairForm {
        PairForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn can_be_parameter(&self) -> bool {
        match self.first.clone() {
            PairFormValue::Ignore(_)
            | PairFormValue::Empty(_)
            | PairFormValue::Panic(_)
            | PairFormValue::Atomic(_)
            | PairFormValue::ValueKeyword(_)
            | PairFormValue::TypeKeyword(_)
            | PairFormValue::TypeSymbol(_)
            | PairFormValue::TypePathSymbol(_)
            | PairFormValue::FunForm(_)
            | PairFormValue::Type(_)
            | PairFormValue::CaseForm(_)
            | PairFormValue::LetForm(_)
            | PairFormValue::AppForm(_) => return false,
            PairFormValue::PairForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::MapForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::ArrForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::VecForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::ListForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            _ => {}
        }

        match self.second.clone() {
            PairFormValue::Ignore(_)
            | PairFormValue::Empty(_)
            | PairFormValue::Panic(_)
            | PairFormValue::Atomic(_)
            | PairFormValue::ValueKeyword(_)
            | PairFormValue::TypeKeyword(_)
            | PairFormValue::TypeSymbol(_)
            | PairFormValue::TypePathSymbol(_)
            | PairFormValue::FunForm(_)
            | PairFormValue::Type(_)
            | PairFormValue::CaseForm(_)
            | PairFormValue::LetForm(_)
            | PairFormValue::AppForm(_) => return false,
            PairFormValue::PairForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::MapForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::ArrForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::VecForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            PairFormValue::ListForm(form) => {
                if !form.can_be_parameter() {
                    return false;
                }
            }
            _ => {}
        }

        true
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        match self.first.clone() {
            PairFormValue::Type(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::MapForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::VecForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::ArrForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::ListForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::PairForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        match self.second.clone() {
            PairFormValue::Type(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::MapForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::VecForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::ArrForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::ListForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::PairForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            PairFormValue::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.first.clone() {
            PairFormValue::ValueSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::TypeSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::ValuePathSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::TypePathSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::Type(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::MapForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::VecForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::ArrForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::ListForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::PairForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::CaseForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        match self.second.clone() {
            PairFormValue::ValueSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::TypeSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::ValuePathSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::TypePathSymbol(value) => {
                vars.push(value);
            }
            PairFormValue::Type(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::MapForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::VecForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::ArrForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::ListForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::PairForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::CaseForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            PairFormValue::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<PairForm> {
        if form.head.to_string() != "pair" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a pair keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected two values".into(),
            }));
        }

        let mut pair = PairForm::new();
        pair.tokens = form.tokens.clone();

        let first = form.tail[0].clone();
        let second = form.tail[1].clone();

        match first {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    pair.first = PairFormValue::Empty(value);
                }
                SimpleValue::Atomic(_) => {
                    pair.first = PairFormValue::Atomic(value);
                }
                SimpleValue::ValueKeyword(_) => {
                    pair.first = PairFormValue::ValueKeyword(value);
                }
                SimpleValue::TypeKeyword(_) => {
                    pair.first = PairFormValue::TypeKeyword(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    pair.first = PairFormValue::ValueSymbol(value);
                }
                SimpleValue::TypeSymbol(_) => {
                    pair.first = PairFormValue::TypeSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    pair.first = PairFormValue::ValuePathSymbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    pair.first = PairFormValue::TypePathSymbol(value);
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
                    pair.first = PairFormValue::Type(Box::new(form));
                } else if let Ok(form) = MapForm::from_form(&form) {
                    pair.first = PairFormValue::MapForm(Box::new(form));
                } else if let Ok(form) = VecForm::from_form(&form) {
                    pair.first = PairFormValue::VecForm(Box::new(form));
                } else if let Ok(form) = ArrForm::from_form(&form) {
                    pair.first = PairFormValue::ArrForm(Box::new(form));
                } else if let Ok(form) = ListForm::from_form(&form) {
                    pair.first = PairFormValue::ListForm(Box::new(form));
                } else if let Ok(form) = PairForm::from_form(&form) {
                    pair.first = PairFormValue::PairForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    pair.first = PairFormValue::FunForm(Box::new(form));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    pair.first = PairFormValue::CaseForm(Box::new(form));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    pair.first = PairFormValue::LetForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    pair.first = PairFormValue::AppForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected form".into(),
                    }));
                }
            }
        }

        match second {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    pair.second = PairFormValue::Empty(value);
                }
                SimpleValue::Atomic(_) => {
                    pair.second = PairFormValue::Atomic(value);
                }
                SimpleValue::ValueKeyword(_) => {
                    pair.second = PairFormValue::ValueKeyword(value);
                }
                SimpleValue::TypeKeyword(_) => {
                    pair.second = PairFormValue::TypeKeyword(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    pair.second = PairFormValue::ValueSymbol(value);
                }
                SimpleValue::TypeSymbol(_) => {
                    pair.second = PairFormValue::TypeSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    pair.second = PairFormValue::ValuePathSymbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    pair.second = PairFormValue::TypePathSymbol(value);
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
                    pair.second = PairFormValue::Type(Box::new(form));
                } else if let Ok(form) = MapForm::from_form(&form) {
                    pair.second = PairFormValue::MapForm(Box::new(form));
                } else if let Ok(form) = VecForm::from_form(&form) {
                    pair.second = PairFormValue::VecForm(Box::new(form));
                } else if let Ok(form) = ArrForm::from_form(&form) {
                    pair.second = PairFormValue::ArrForm(Box::new(form));
                } else if let Ok(form) = ListForm::from_form(&form) {
                    pair.second = PairFormValue::ListForm(Box::new(form));
                } else if let Ok(form) = PairForm::from_form(&form) {
                    pair.second = PairFormValue::PairForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    pair.second = PairFormValue::FunForm(Box::new(form));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    pair.second = PairFormValue::CaseForm(Box::new(form));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    pair.second = PairFormValue::LetForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    pair.second = PairFormValue::AppForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected form".into(),
                    }));
                }
            }
        }

        Ok(pair)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<PairForm> {
        let form = Form::from_tokens(tokens)?;

        PairForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<PairForm> {
        let tokens = Tokens::from_str(s)?;

        PairForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(pair {} {})",
            self.first.to_string(),
            self.second.to_string()
        )
    }
}

impl fmt::Display for PairForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for PairForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pair_form_from_str() {
        use super::PairForm;

        let mut s = "(pair a A)";

        let mut res = PairForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.first.to_string(), "a".to_string());
        assert_eq!(form.second.to_string(), "A".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(pair moduleX.X y)";

        res = PairForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.first.to_string(), "moduleX.X".to_string());
        assert_eq!(form.second.to_string(), "y".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(pair 0 (Fun A B))";

        res = PairForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.first.to_string(), "0".to_string());
        assert_eq!(form.second.to_string(), "(Fun A B)".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
