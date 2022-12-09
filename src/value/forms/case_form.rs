use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::app_form::AppForm;
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::forms::fun_form::FunForm;
use crate::value::forms::let_form::LetForm;
use crate::value::forms::pair_form::PairForm;
use crate::value::SimpleValue;
use crate::value::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum CaseFormVariable {
    Empty(SimpleValue),
    Atomic(SimpleValue),
    ValueSymbol(SimpleValue),
    AppForm(Box<AppForm>),
    LetForm(Box<LetForm>),
    CaseForm(Box<CaseForm>),
}

impl Default for CaseFormVariable {
    fn default() -> CaseFormVariable {
        CaseFormVariable::Empty(SimpleValue::new())
    }
}

impl CaseFormVariable {
    pub fn file(&self) -> String {
        match self {
            CaseFormVariable::Empty(empty) => empty.file(),
            CaseFormVariable::Atomic(atomic) => atomic.file(),
            CaseFormVariable::ValueSymbol(symbol) => symbol.file(),
            CaseFormVariable::AppForm(form) => form.file(),
            CaseFormVariable::LetForm(form) => form.file(),
            CaseFormVariable::CaseForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            CaseFormVariable::Empty(empty) => empty.loc(),
            CaseFormVariable::Atomic(atomic) => atomic.loc(),
            CaseFormVariable::ValueSymbol(symbol) => symbol.loc(),
            CaseFormVariable::AppForm(form) => form.loc(),
            CaseFormVariable::LetForm(form) => form.loc(),
            CaseFormVariable::CaseForm(form) => form.loc(),
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        match self.clone() {
            CaseFormVariable::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            CaseFormVariable::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            CaseFormVariable::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        match self.clone() {
            CaseFormVariable::ValueSymbol(value) => {
                value_vars.push(value);
            }
            CaseFormVariable::AppForm(form) => {
                value_vars.extend(form.all_value_variables());
            }
            CaseFormVariable::LetForm(form) => {
                value_vars.extend(form.all_value_variables());
            }
            CaseFormVariable::CaseForm(form) => {
                value_vars.extend(form.all_value_variables());
            }
            _ => {}
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        match self.clone() {
            CaseFormVariable::AppForm(form) => {
                type_vars.extend(form.all_type_variables());
            }
            CaseFormVariable::LetForm(form) => {
                type_vars.extend(form.all_type_variables());
            }
            CaseFormVariable::CaseForm(form) => {
                type_vars.extend(form.all_type_variables());
            }
            _ => {}
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.clone() {
            CaseFormVariable::ValueSymbol(value) => {
                vars.push(value);
            }
            CaseFormVariable::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormVariable::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormVariable::CaseForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        vars
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormVariable::Empty(_) => "()".into(),
            CaseFormVariable::Atomic(atomic) => atomic.to_string(),
            CaseFormVariable::ValueSymbol(symbol) => symbol.to_string(),
            CaseFormVariable::AppForm(form) => form.to_string(),
            CaseFormVariable::LetForm(form) => form.to_string(),
            CaseFormVariable::CaseForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for CaseFormVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum CaseFormMatchCase {
    Empty(SimpleValue),
    Atomic(SimpleValue),
    TypeKeyword(SimpleValue),
    TypeSymbol(SimpleValue),
    ValueSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
}

impl CaseFormMatchCase {
    pub fn file(&self) -> String {
        match self {
            CaseFormMatchCase::Empty(empty) => empty.file(),
            CaseFormMatchCase::Atomic(atomic) => atomic.file(),
            CaseFormMatchCase::TypeKeyword(keyword) => keyword.file(),
            CaseFormMatchCase::TypeSymbol(symbol) => symbol.file(),
            CaseFormMatchCase::ValueSymbol(symbol) => symbol.file(),
            CaseFormMatchCase::TypePathSymbol(symbol) => symbol.file(),
            CaseFormMatchCase::ValuePathSymbol(symbol) => symbol.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            CaseFormMatchCase::Empty(empty) => empty.loc(),
            CaseFormMatchCase::Atomic(atomic) => atomic.loc(),
            CaseFormMatchCase::TypeKeyword(keyword) => keyword.loc(),
            CaseFormMatchCase::TypeSymbol(symbol) => symbol.loc(),
            CaseFormMatchCase::ValueSymbol(symbol) => symbol.loc(),
            CaseFormMatchCase::TypePathSymbol(symbol) => symbol.loc(),
            CaseFormMatchCase::ValuePathSymbol(symbol) => symbol.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchCase::Empty(_) => "()".into(),
            CaseFormMatchCase::Atomic(atomic) => atomic.to_string(),
            CaseFormMatchCase::TypeKeyword(keyword) => keyword.to_string(),
            CaseFormMatchCase::TypeSymbol(symbol) => symbol.to_string(),
            CaseFormMatchCase::ValueSymbol(symbol) => symbol.to_string(),
            CaseFormMatchCase::TypePathSymbol(symbol) => symbol.to_string(),
            CaseFormMatchCase::ValuePathSymbol(symbol) => symbol.to_string(),
        }
    }
}

impl fmt::Display for CaseFormMatchCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for CaseFormMatchCase {
    fn default() -> CaseFormMatchCase {
        CaseFormMatchCase::Empty(SimpleValue::new())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum CaseFormMatchAction {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueKeyword(SimpleValue),
    ValueSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    PairForm(Box<PairForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
}

impl CaseFormMatchAction {
    pub fn file(&self) -> String {
        match self {
            CaseFormMatchAction::Ignore(ignore) => ignore.file(),
            CaseFormMatchAction::Empty(empty) => empty.file(),
            CaseFormMatchAction::Panic(panic) => panic.file(),
            CaseFormMatchAction::Atomic(atomic) => atomic.file(),
            CaseFormMatchAction::ValueKeyword(keyword) => keyword.file(),
            CaseFormMatchAction::ValueSymbol(symbol) => symbol.file(),
            CaseFormMatchAction::ValuePathSymbol(symbol) => symbol.file(),
            CaseFormMatchAction::PairForm(form) => form.file(),
            CaseFormMatchAction::FunForm(form) => form.file(),
            CaseFormMatchAction::LetForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            CaseFormMatchAction::Ignore(ignore) => ignore.loc(),
            CaseFormMatchAction::Empty(empty) => empty.loc(),
            CaseFormMatchAction::Panic(panic) => panic.loc(),
            CaseFormMatchAction::Atomic(atomic) => atomic.loc(),
            CaseFormMatchAction::ValueKeyword(keyword) => keyword.loc(),
            CaseFormMatchAction::ValueSymbol(symbol) => symbol.loc(),
            CaseFormMatchAction::ValuePathSymbol(symbol) => symbol.loc(),
            CaseFormMatchAction::PairForm(form) => form.loc(),
            CaseFormMatchAction::FunForm(form) => form.loc(),
            CaseFormMatchAction::LetForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchAction::Ignore(_) => "_".into(),
            CaseFormMatchAction::Empty(_) => "_".into(),
            CaseFormMatchAction::Panic(_) => "panic".into(),
            CaseFormMatchAction::Atomic(atomic) => atomic.to_string(),
            CaseFormMatchAction::ValueKeyword(keyword) => keyword.to_string(),
            CaseFormMatchAction::ValueSymbol(symbol) => symbol.to_string(),
            CaseFormMatchAction::ValuePathSymbol(symbol) => symbol.to_string(),
            CaseFormMatchAction::PairForm(form) => form.to_string(),
            CaseFormMatchAction::FunForm(form) => form.to_string(),
            CaseFormMatchAction::LetForm(form) => form.to_string(),
        }
    }
}

impl Default for CaseFormMatchAction {
    fn default() -> CaseFormMatchAction {
        CaseFormMatchAction::Empty(SimpleValue::new())
    }
}

impl fmt::Display for CaseFormMatchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct CaseFormMatch {
    pub tokens: Box<Tokens>,
    pub case: CaseFormMatchCase,
    pub action: CaseFormMatchAction,
}

impl CaseFormMatch {
    pub fn new() -> CaseFormMatch {
        CaseFormMatch::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        match self.action.clone() {
            CaseFormMatchAction::PairForm(form) => {
                params.extend(form.all_parameters());
            }
            CaseFormMatchAction::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            CaseFormMatchAction::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        match self.action.clone() {
            CaseFormMatchAction::ValueSymbol(value) => {
                value_vars.push(value);
            }
            CaseFormMatchAction::ValuePathSymbol(value) => {
                value_vars.push(value);
            }
            CaseFormMatchAction::PairForm(form) => {
                value_vars.extend(form.all_value_variables());
            }
            CaseFormMatchAction::FunForm(form) => {
                value_vars.extend(form.all_value_variables());
            }
            CaseFormMatchAction::LetForm(form) => {
                value_vars.extend(form.all_value_variables());
            }
            _ => {}
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        match self.action.clone() {
            CaseFormMatchAction::PairForm(form) => {
                type_vars.extend(form.all_type_variables());
            }
            CaseFormMatchAction::FunForm(form) => {
                type_vars.extend(form.all_type_variables());
            }
            CaseFormMatchAction::LetForm(form) => {
                type_vars.extend(form.all_type_variables());
            }
            _ => {}
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.action.clone() {
            CaseFormMatchAction::ValueSymbol(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::ValuePathSymbol(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::PairForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormMatchAction::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormMatchAction::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<CaseFormMatch> {
        if form.head.to_string() != "match" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a match keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a symbol, an atomic or application followed by a function".into(),
            }));
        }

        let mut case_match = CaseFormMatch::new();
        case_match.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    case_match.case = CaseFormMatchCase::Empty(value);
                }
                SimpleValue::Atomic(_) => {
                    case_match.case = CaseFormMatchCase::Atomic(value);
                }
                SimpleValue::TypeKeyword(_) => {
                    case_match.case = CaseFormMatchCase::TypeKeyword(value);
                }
                SimpleValue::TypeSymbol(_) => {
                    case_match.case = CaseFormMatchCase::TypeSymbol(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    case_match.case = CaseFormMatchCase::ValueSymbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    case_match.case = CaseFormMatchCase::TypePathSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    case_match.case = CaseFormMatchCase::ValuePathSymbol(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: x.loc(),
                    desc: "unexpected form".into(),
                }));
            }
        }

        match form.tail[1].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Ignore(_) => {
                    case_match.action = CaseFormMatchAction::Ignore(value);
                }
                SimpleValue::Empty(_) => {
                    case_match.action = CaseFormMatchAction::Empty(value);
                }
                SimpleValue::Panic(_) => {
                    case_match.action = CaseFormMatchAction::Panic(value);
                }
                SimpleValue::Atomic(_) => {
                    case_match.action = CaseFormMatchAction::Atomic(value);
                }
                SimpleValue::ValueKeyword(_) => {
                    case_match.action = CaseFormMatchAction::ValueKeyword(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    case_match.action = CaseFormMatchAction::ValueSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    case_match.action = CaseFormMatchAction::ValuePathSymbol(value);
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
                    case_match.action = CaseFormMatchAction::PairForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::FunForm(Box::new(form));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::LetForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected form".into(),
                    }));
                }
            }
        }

        Ok(case_match)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<CaseFormMatch> {
        let form = Form::from_tokens(tokens)?;

        CaseFormMatch::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<CaseFormMatch> {
        let tokens = Tokens::from_str(s)?;

        CaseFormMatch::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(match {} {})",
            self.case.to_string(),
            self.action.to_string()
        )
    }
}

impl fmt::Display for CaseFormMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct CaseForm {
    pub tokens: Box<Tokens>,
    pub variable: CaseFormVariable,
    pub matches: Vec<CaseFormMatch>,
}

impl CaseForm {
    pub fn new() -> CaseForm {
        CaseForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn matches_to_string(&self) -> String {
        self.matches
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        params.extend(self.variable.all_parameters());

        for branch in self.matches.iter() {
            let new_params = branch
                .all_parameters()
                .iter()
                .filter(|bv| !params.iter().any(|v| v.to_string() == bv.to_string()))
                .map(|v| v.to_owned())
                .collect::<Vec<SimpleValue>>();

            params.extend(new_params);
        }

        params
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        value_vars.extend(self.variable.all_value_variables());

        for branch in self.matches.iter() {
            let new_value_vars = branch
                .all_value_variables()
                .iter()
                .filter(|bv| !value_vars.iter().any(|v| v.to_string() == bv.to_string()))
                .map(|v| v.to_owned())
                .collect::<Vec<SimpleValue>>();

            value_vars.extend(new_value_vars);
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        type_vars.extend(self.variable.all_type_variables());

        for branch in self.matches.iter() {
            let new_type_vars = branch
                .all_type_variables()
                .iter()
                .filter(|bv| !type_vars.iter().any(|v| v.to_string() == bv.to_string()))
                .map(|v| v.to_owned())
                .collect::<Vec<Type>>();

            type_vars.extend(new_type_vars);
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        vars.extend(self.variable.all_variables());

        for branch in self.matches.iter() {
            let new_vars = branch
                .all_variables()
                .iter()
                .filter(|bv| !vars.iter().any(|v| v.to_string() == bv.to_string()))
                .map(|v| v.to_owned())
                .collect::<Vec<SimpleValue>>();

            vars.extend(new_vars);
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<CaseForm> {
        if form.head.to_string() != "case" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a case keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a case form parameter and at least one match branch".into(),
            }));
        }

        let mut case = CaseForm::new();
        case.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    case.variable = CaseFormVariable::Empty(value);
                }
                SimpleValue::Atomic(_) => {
                    case.variable = CaseFormVariable::Atomic(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    case.variable = CaseFormVariable::ValueSymbol(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(form) = LetForm::from_form(&form) {
                    case.variable = CaseFormVariable::LetForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    case.variable = CaseFormVariable::AppForm(Box::new(form));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    case.variable = CaseFormVariable::CaseForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected form".into(),
                    }));
                }
            }
        }

        for param in form.tail[1..].iter() {
            match param.clone() {
                FormTailElement::Form(form) => {
                    if let Ok(form) = CaseFormMatch::from_form(&form) {
                        case.matches.push(form);
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected a case match form".into(),
                        }));
                    }
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a case match form".into(),
                    }));
                }
            }
        }

        Ok(case)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<CaseForm> {
        let form = Form::from_tokens(tokens)?;

        CaseForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<CaseForm> {
        let tokens = Tokens::from_str(s)?;

        CaseForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(case {} {})",
            self.variable.to_string(),
            self.matches_to_string(),
        )
    }
}

impl fmt::Display for CaseForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for CaseForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_form_match_from_str() {
        use super::CaseFormMatch;

        let mut s = "(match True \"True\")";

        let mut res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        let mut case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "True".to_string());
        assert_eq!(case_match.action.to_string(), "\"True\"".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match True id)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "True".to_string());
        assert_eq!(case_match.action.to_string(), "id".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match True (fun t \"True\"))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "True".to_string());
        assert_eq!(
            case_match.action.to_string(),
            "(fun t \"True\")".to_string()
        );
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match 0 (fun n \"0\"))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "0".to_string());
        assert_eq!(case_match.action.to_string(), "(fun n \"0\")".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match Char (fun t \"Char\"))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "Char".to_string());
        assert_eq!(
            case_match.action.to_string(),
            "(fun t \"Char\")".to_string()
        );
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match '0' (fun c 0))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "'0'".to_string());
        assert_eq!(case_match.action.to_string(), "(fun c 0)".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match () _)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "()".to_string());
        assert_eq!(case_match.action.to_string(), "_".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match T id)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "T".to_string());
        assert_eq!(case_match.action.to_string(), "id".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match E panic)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "E".to_string());
        assert_eq!(case_match.action.to_string(), "panic".to_string());
        assert_eq!(case_match.to_string(), s.to_string());

        s = "(match E (let (val f (fun () panic)) (f ())))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        case_match = res.unwrap();

        assert_eq!(case_match.case.to_string(), "E".to_string());
        assert_eq!(
            case_match.action.to_string(),
            "(let (val f (fun () panic)) (f ()))".to_string()
        );
        assert_eq!(case_match.to_string(), s.to_string());
    }

    #[test]
    fn case_form_from_str() {
        use super::CaseForm;

        let mut s = "(case t (match True (fun t \"True\")) (match False (fun f \"False\")))";

        let mut res = CaseForm::from_str(s);

        assert!(res.is_ok());

        let mut case = res.unwrap();

        assert_eq!(case.variable.to_string(), "t".to_string());
        assert_eq!(
            case.matches_to_string(),
            "(match True (fun t \"True\")) (match False (fun f \"False\"))".to_string()
        );
        assert_eq!(case.to_string(), s.to_string());

        s = "(case (id bool) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "(id bool)".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case (let (id bool)) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "(let (id bool))".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case bool (match True (fun t \"True\")) (match False _))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "bool".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case res (match T id) (match E panic))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "res".to_string());
        assert_eq!(
            case.matches_to_string(),
            "(match T id) (match E panic)".to_string()
        );
        assert_eq!(case.to_string(), s.to_string());
    }
}
