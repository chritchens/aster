use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::app_form::AppForm;
use crate::value::forms::case_form::CaseForm;
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::forms::fun_form::FunForm;
use crate::value::forms::let_form::LetForm;
use crate::value::forms::list_form::ListForm;
use crate::value::forms::map_form::MapForm;
use crate::value::forms::pair_form::PairForm;
use crate::value::forms::vec_form::VecForm;
use crate::value::types::Type;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum ArrFormValue {
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
    VecForm(Box<VecForm>),
    MapForm(Box<MapForm>),
    ArrForm(Box<ArrForm>),
}

impl Default for ArrFormValue {
    fn default() -> ArrFormValue {
        ArrFormValue::Empty(SimpleValue::new())
    }
}

impl ArrFormValue {
    pub fn file(&self) -> String {
        match self {
            ArrFormValue::Ignore(ignore) => ignore.file(),
            ArrFormValue::Empty(empty) => empty.file(),
            ArrFormValue::Panic(panic) => panic.file(),
            ArrFormValue::Atomic(atomic) => atomic.file(),
            ArrFormValue::ValueKeyword(keyword) => keyword.file(),
            ArrFormValue::TypeKeyword(keyword) => keyword.file(),
            ArrFormValue::ValueSymbol(symbol) => symbol.file(),
            ArrFormValue::TypeSymbol(symbol) => symbol.file(),
            ArrFormValue::ValuePathSymbol(symbol) => symbol.file(),
            ArrFormValue::TypePathSymbol(symbol) => symbol.file(),
            ArrFormValue::Type(form) => form.file(),
            ArrFormValue::FunForm(form) => form.file(),
            ArrFormValue::CaseForm(form) => form.file(),
            ArrFormValue::LetForm(form) => form.file(),
            ArrFormValue::AppForm(form) => form.file(),
            ArrFormValue::PairForm(form) => form.file(),
            ArrFormValue::ListForm(form) => form.file(),
            ArrFormValue::VecForm(form) => form.file(),
            ArrFormValue::MapForm(form) => form.file(),
            ArrFormValue::ArrForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ArrFormValue::Ignore(ignore) => ignore.loc(),
            ArrFormValue::Empty(empty) => empty.loc(),
            ArrFormValue::Panic(panic) => panic.loc(),
            ArrFormValue::Atomic(atomic) => atomic.loc(),
            ArrFormValue::ValueKeyword(keyword) => keyword.loc(),
            ArrFormValue::TypeKeyword(keyword) => keyword.loc(),
            ArrFormValue::ValueSymbol(symbol) => symbol.loc(),
            ArrFormValue::TypeSymbol(symbol) => symbol.loc(),
            ArrFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            ArrFormValue::TypePathSymbol(symbol) => symbol.loc(),
            ArrFormValue::Type(form) => form.loc(),
            ArrFormValue::FunForm(form) => form.loc(),
            ArrFormValue::CaseForm(form) => form.loc(),
            ArrFormValue::LetForm(form) => form.loc(),
            ArrFormValue::AppForm(form) => form.loc(),
            ArrFormValue::PairForm(form) => form.loc(),
            ArrFormValue::ListForm(form) => form.loc(),
            ArrFormValue::VecForm(form) => form.loc(),
            ArrFormValue::MapForm(form) => form.loc(),
            ArrFormValue::ArrForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ArrFormValue::Ignore(_) => "_".into(),
            ArrFormValue::Empty(_) => "()".into(),
            ArrFormValue::Panic(_) => "panic".into(),
            ArrFormValue::Atomic(atomic) => atomic.to_string(),
            ArrFormValue::ValueKeyword(keyword) => keyword.to_string(),
            ArrFormValue::TypeKeyword(keyword) => keyword.to_string(),
            ArrFormValue::ValueSymbol(symbol) => symbol.to_string(),
            ArrFormValue::TypeSymbol(symbol) => symbol.to_string(),
            ArrFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            ArrFormValue::TypePathSymbol(symbol) => symbol.to_string(),
            ArrFormValue::Type(form) => form.to_string(),
            ArrFormValue::FunForm(form) => form.to_string(),
            ArrFormValue::CaseForm(form) => form.to_string(),
            ArrFormValue::LetForm(form) => form.to_string(),
            ArrFormValue::AppForm(form) => form.to_string(),
            ArrFormValue::PairForm(form) => form.to_string(),
            ArrFormValue::ListForm(form) => form.to_string(),
            ArrFormValue::VecForm(form) => form.to_string(),
            ArrFormValue::MapForm(form) => form.to_string(),
            ArrFormValue::ArrForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ArrFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ArrForm {
    pub tokens: Box<Tokens>,
    pub values: Vec<ArrFormValue>,
}

impl ArrForm {
    pub fn new() -> ArrForm {
        ArrForm::default()
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
                ArrFormValue::Ignore(_)
                | ArrFormValue::Empty(_)
                | ArrFormValue::Atomic(_)
                | ArrFormValue::ValueKeyword(_)
                | ArrFormValue::TypeKeyword(_)
                | ArrFormValue::TypeSymbol(_)
                | ArrFormValue::TypePathSymbol(_)
                | ArrFormValue::FunForm(_)
                | ArrFormValue::Type(_)
                | ArrFormValue::CaseForm(_)
                | ArrFormValue::LetForm(_)
                | ArrFormValue::AppForm(_) => return false,
                ArrFormValue::PairForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ArrFormValue::MapForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ArrFormValue::VecForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ArrFormValue::ListForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ArrFormValue::ArrForm(form) => {
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
                ArrFormValue::FunForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::CaseForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::LetForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::AppForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::PairForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::ListForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::VecForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::MapForm(form) => {
                    params.extend(form.all_parameters());
                }
                ArrFormValue::ArrForm(form) => {
                    params.extend(form.all_parameters());
                }
                _ => {}
            }
        }

        params
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        for value in self.values.iter() {
            match value.clone() {
                ArrFormValue::ValueSymbol(value) => {
                    value_vars.push(value);
                }
                ArrFormValue::ValuePathSymbol(value) => {
                    value_vars.push(value);
                }
                ArrFormValue::FunForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::CaseForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::LetForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::AppForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::PairForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::ListForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::VecForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::MapForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ArrFormValue::ArrForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                _ => {}
            }
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        for value in self.values.iter() {
            match value.clone() {
                ArrFormValue::TypeSymbol(value) => {
                    type_vars.push(Type::from_simple_value(&value).unwrap());
                }
                ArrFormValue::TypePathSymbol(value) => {
                    type_vars.push(Type::from_simple_value(&value).unwrap());
                }
                ArrFormValue::Type(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::FunForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::CaseForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::LetForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::AppForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::PairForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::ListForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::VecForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::MapForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ArrFormValue::ArrForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                _ => {}
            }
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for value in self.values.iter() {
            match value.clone() {
                ArrFormValue::ValueSymbol(value) => {
                    vars.push(value);
                }
                ArrFormValue::TypeSymbol(value) => {
                    vars.push(value);
                }
                ArrFormValue::ValuePathSymbol(value) => {
                    vars.push(value);
                }
                ArrFormValue::TypePathSymbol(value) => {
                    vars.push(value);
                }
                ArrFormValue::Type(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::FunForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::CaseForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::LetForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::AppForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::PairForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::ListForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::VecForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::MapForm(form) => {
                    vars.extend(form.all_variables());
                }
                ArrFormValue::ArrForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<ArrForm> {
        if form.head.to_string() != "arr" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a arr keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least two values".into(),
            }));
        }

        let mut arr = ArrForm::new();
        arr.tokens = form.tokens.clone();

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        arr.values.push(ArrFormValue::Ignore(value));
                    }
                    SimpleValue::Empty(_) => {
                        arr.values.push(ArrFormValue::Empty(value));
                    }
                    SimpleValue::Atomic(_) => {
                        arr.values.push(ArrFormValue::Atomic(value));
                    }
                    SimpleValue::ValueKeyword(_) => {
                        arr.values.push(ArrFormValue::ValueKeyword(value));
                    }
                    SimpleValue::TypeKeyword(_) => {
                        arr.values.push(ArrFormValue::TypeKeyword(value));
                    }
                    SimpleValue::ValueSymbol(_) => {
                        arr.values.push(ArrFormValue::ValueSymbol(value));
                    }
                    SimpleValue::TypeSymbol(_) => {
                        arr.values.push(ArrFormValue::TypeSymbol(value));
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        arr.values.push(ArrFormValue::ValuePathSymbol(value));
                    }
                    SimpleValue::TypePathSymbol(_) => {
                        arr.values.push(ArrFormValue::TypePathSymbol(value));
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
                        arr.values.push(ArrFormValue::Type(Box::new(form)));
                    } else if let Ok(form) = PairForm::from_form(&form) {
                        arr.values.push(ArrFormValue::PairForm(Box::new(form)));
                    } else if let Ok(form) = ListForm::from_form(&form) {
                        arr.values.push(ArrFormValue::ListForm(Box::new(form)));
                    } else if let Ok(form) = VecForm::from_form(&form) {
                        arr.values.push(ArrFormValue::VecForm(Box::new(form)));
                    } else if let Ok(form) = MapForm::from_form(&form) {
                        arr.values.push(ArrFormValue::MapForm(Box::new(form)));
                    } else if let Ok(form) = ArrForm::from_form(&form) {
                        arr.values.push(ArrFormValue::ArrForm(Box::new(form)));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        arr.values.push(ArrFormValue::FunForm(Box::new(form)));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        arr.values.push(ArrFormValue::CaseForm(Box::new(form)));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        arr.values.push(ArrFormValue::LetForm(Box::new(form)));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        arr.values.push(ArrFormValue::AppForm(Box::new(form)))
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        Ok(arr)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ArrForm> {
        let form = Form::from_tokens(tokens)?;

        ArrForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ArrForm> {
        let tokens = Tokens::from_str(s)?;

        ArrForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(arr {})", self.values_to_string())
    }
}

impl fmt::Display for ArrForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for ArrForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn arr_form_from_str() {
        use super::ArrForm;

        let mut s = "(arr a A)";

        let mut res = ArrForm::from_str(s);

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

        s = "(arr moduleX.X y)";

        res = ArrForm::from_str(s);

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

        s = "(arr 0 (Fun A B))";

        res = ArrForm::from_str(s);

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
