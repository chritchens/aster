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
use crate::value::forms::map_form::MapForm;
use crate::value::forms::pair_form::PairForm;
use crate::value::forms::vec_form::VecForm;
use crate::value::types::Type;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum ListFormValue {
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
    ArrForm(Box<ArrForm>),
    VecForm(Box<VecForm>),
    MapForm(Box<MapForm>),
    ListForm(Box<ListForm>),
}

impl Default for ListFormValue {
    fn default() -> ListFormValue {
        ListFormValue::Empty(SimpleValue::new())
    }
}

impl ListFormValue {
    pub fn file(&self) -> String {
        match self {
            ListFormValue::Ignore(ignore) => ignore.file(),
            ListFormValue::Empty(empty) => empty.file(),
            ListFormValue::Panic(panic) => panic.file(),
            ListFormValue::Atomic(atomic) => atomic.file(),
            ListFormValue::ValueKeyword(keyword) => keyword.file(),
            ListFormValue::TypeKeyword(keyword) => keyword.file(),
            ListFormValue::ValueSymbol(symbol) => symbol.file(),
            ListFormValue::TypeSymbol(symbol) => symbol.file(),
            ListFormValue::ValuePathSymbol(symbol) => symbol.file(),
            ListFormValue::TypePathSymbol(symbol) => symbol.file(),
            ListFormValue::Type(form) => form.file(),
            ListFormValue::FunForm(form) => form.file(),
            ListFormValue::CaseForm(form) => form.file(),
            ListFormValue::LetForm(form) => form.file(),
            ListFormValue::AppForm(form) => form.file(),
            ListFormValue::PairForm(form) => form.file(),
            ListFormValue::ArrForm(form) => form.file(),
            ListFormValue::VecForm(form) => form.file(),
            ListFormValue::MapForm(form) => form.file(),
            ListFormValue::ListForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ListFormValue::Ignore(ignore) => ignore.loc(),
            ListFormValue::Empty(empty) => empty.loc(),
            ListFormValue::Panic(panic) => panic.loc(),
            ListFormValue::Atomic(atomic) => atomic.loc(),
            ListFormValue::ValueKeyword(keyword) => keyword.loc(),
            ListFormValue::TypeKeyword(keyword) => keyword.loc(),
            ListFormValue::ValueSymbol(symbol) => symbol.loc(),
            ListFormValue::TypeSymbol(symbol) => symbol.loc(),
            ListFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            ListFormValue::TypePathSymbol(symbol) => symbol.loc(),
            ListFormValue::Type(form) => form.loc(),
            ListFormValue::FunForm(form) => form.loc(),
            ListFormValue::CaseForm(form) => form.loc(),
            ListFormValue::LetForm(form) => form.loc(),
            ListFormValue::AppForm(form) => form.loc(),
            ListFormValue::PairForm(form) => form.loc(),
            ListFormValue::ArrForm(form) => form.loc(),
            ListFormValue::VecForm(form) => form.loc(),
            ListFormValue::MapForm(form) => form.loc(),
            ListFormValue::ListForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ListFormValue::Ignore(_) => "_".into(),
            ListFormValue::Empty(_) => "()".into(),
            ListFormValue::Panic(_) => "panic".into(),
            ListFormValue::Atomic(atomic) => atomic.to_string(),
            ListFormValue::ValueKeyword(keyword) => keyword.to_string(),
            ListFormValue::TypeKeyword(keyword) => keyword.to_string(),
            ListFormValue::ValueSymbol(symbol) => symbol.to_string(),
            ListFormValue::TypeSymbol(symbol) => symbol.to_string(),
            ListFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            ListFormValue::TypePathSymbol(symbol) => symbol.to_string(),
            ListFormValue::Type(form) => form.to_string(),
            ListFormValue::FunForm(form) => form.to_string(),
            ListFormValue::CaseForm(form) => form.to_string(),
            ListFormValue::LetForm(form) => form.to_string(),
            ListFormValue::AppForm(form) => form.to_string(),
            ListFormValue::PairForm(form) => form.to_string(),
            ListFormValue::ArrForm(form) => form.to_string(),
            ListFormValue::VecForm(form) => form.to_string(),
            ListFormValue::MapForm(form) => form.to_string(),
            ListFormValue::ListForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ListFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ListForm {
    pub tokens: Box<Tokens>,
    pub values: Vec<ListFormValue>,
}

impl ListForm {
    pub fn new() -> ListForm {
        ListForm::default()
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
                ListFormValue::Ignore(_)
                | ListFormValue::Empty(_)
                | ListFormValue::Atomic(_)
                | ListFormValue::ValueKeyword(_)
                | ListFormValue::TypeKeyword(_)
                | ListFormValue::TypeSymbol(_)
                | ListFormValue::TypePathSymbol(_)
                | ListFormValue::FunForm(_)
                | ListFormValue::Type(_)
                | ListFormValue::CaseForm(_)
                | ListFormValue::LetForm(_)
                | ListFormValue::AppForm(_) => return false,
                ListFormValue::PairForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ListFormValue::MapForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ListFormValue::ArrForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ListFormValue::VecForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
                ListFormValue::ListForm(form) => {
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
                ListFormValue::FunForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::CaseForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::LetForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::AppForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::PairForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::ArrForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::MapForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::VecForm(form) => {
                    params.extend(form.all_parameters());
                }
                ListFormValue::ListForm(form) => {
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
                ListFormValue::ValueSymbol(value) => {
                    value_vars.push(value);
                }
                ListFormValue::ValuePathSymbol(value) => {
                    value_vars.push(value);
                }
                ListFormValue::FunForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::CaseForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::LetForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::AppForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::PairForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::ArrForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::MapForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::VecForm(form) => {
                    value_vars.extend(form.all_value_variables());
                }
                ListFormValue::ListForm(form) => {
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
                ListFormValue::TypeSymbol(value) => {
                    type_vars.push(Type::from_simple_value(&value).unwrap());
                }
                ListFormValue::TypePathSymbol(value) => {
                    type_vars.push(Type::from_simple_value(&value).unwrap());
                }
                ListFormValue::Type(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::FunForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::CaseForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::LetForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::AppForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::PairForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::ArrForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::MapForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::VecForm(form) => {
                    type_vars.extend(form.all_type_variables());
                }
                ListFormValue::ListForm(form) => {
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
                ListFormValue::ValueSymbol(value) => {
                    vars.push(value);
                }
                ListFormValue::TypeSymbol(value) => {
                    vars.push(value);
                }
                ListFormValue::ValuePathSymbol(value) => {
                    vars.push(value);
                }
                ListFormValue::TypePathSymbol(value) => {
                    vars.push(value);
                }
                ListFormValue::Type(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::FunForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::CaseForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::LetForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::AppForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::PairForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::ArrForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::MapForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::VecForm(form) => {
                    vars.extend(form.all_variables());
                }
                ListFormValue::ListForm(form) => {
                    vars.extend(form.all_variables());
                }
                _ => {}
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<ListForm> {
        if form.head.to_string() != "list" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a list keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least two values".into(),
            }));
        }

        let mut list = ListForm::new();
        list.tokens = form.tokens.clone();

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        list.values.push(ListFormValue::Ignore(value));
                    }
                    SimpleValue::Empty(_) => {
                        list.values.push(ListFormValue::Empty(value));
                    }
                    SimpleValue::Atomic(_) => {
                        list.values.push(ListFormValue::Atomic(value));
                    }
                    SimpleValue::ValueKeyword(_) => {
                        list.values.push(ListFormValue::ValueKeyword(value));
                    }
                    SimpleValue::TypeKeyword(_) => {
                        list.values.push(ListFormValue::TypeKeyword(value));
                    }
                    SimpleValue::ValueSymbol(_) => {
                        list.values.push(ListFormValue::ValueSymbol(value));
                    }
                    SimpleValue::TypeSymbol(_) => {
                        list.values.push(ListFormValue::TypeSymbol(value));
                    }
                    SimpleValue::ValuePathSymbol(_) => {
                        list.values.push(ListFormValue::ValuePathSymbol(value));
                    }
                    SimpleValue::TypePathSymbol(_) => {
                        list.values.push(ListFormValue::TypePathSymbol(value));
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
                        list.values.push(ListFormValue::Type(Box::new(form)));
                    } else if let Ok(form) = PairForm::from_form(&form) {
                        list.values.push(ListFormValue::PairForm(Box::new(form)));
                    } else if let Ok(form) = ArrForm::from_form(&form) {
                        list.values.push(ListFormValue::ArrForm(Box::new(form)));
                    } else if let Ok(form) = MapForm::from_form(&form) {
                        list.values.push(ListFormValue::MapForm(Box::new(form)));
                    } else if let Ok(form) = VecForm::from_form(&form) {
                        list.values.push(ListFormValue::VecForm(Box::new(form)));
                    } else if let Ok(form) = ListForm::from_form(&form) {
                        list.values.push(ListFormValue::ListForm(Box::new(form)));
                    } else if let Ok(form) = FunForm::from_form(&form) {
                        list.values.push(ListFormValue::FunForm(Box::new(form)));
                    } else if let Ok(form) = CaseForm::from_form(&form) {
                        list.values.push(ListFormValue::CaseForm(Box::new(form)));
                    } else if let Ok(form) = LetForm::from_form(&form) {
                        list.values.push(ListFormValue::LetForm(Box::new(form)));
                    } else if let Ok(form) = AppForm::from_form(&form) {
                        list.values.push(ListFormValue::AppForm(Box::new(form)))
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            }
        }

        Ok(list)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ListForm> {
        let form = Form::from_tokens(tokens)?;

        ListForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ListForm> {
        let tokens = Tokens::from_str(s)?;

        ListForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(list {})", self.values_to_string())
    }
}

impl fmt::Display for ListForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for ListForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn list_form_from_str() {
        use super::ListForm;

        let mut s = "(list a A)";

        let mut res = ListForm::from_str(s);

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

        s = "(list moduleX.X y)";

        res = ListForm::from_str(s);

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

        s = "(list 0 (Fun A B))";

        res = ListForm::from_str(s);

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
