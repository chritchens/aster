use crate::error::{Error, SyntacticError};
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::app_form::AppForm;
use crate::value::forms::form::{Form, FormParam};
use crate::value::forms::fun_form::FunForm;
use crate::value::forms::let_form::LetForm;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormParam {
    Empty,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    App(Box<AppForm>),
    Let(Box<LetForm>),
}

impl Default for CaseFormParam {
    fn default() -> CaseFormParam {
        CaseFormParam::Empty
    }
}

impl CaseFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormParam::Empty => "()".into(),
            CaseFormParam::Prim(prim) => prim.clone(),
            CaseFormParam::TypeKeyword(keyword) => keyword.clone(),
            CaseFormParam::TypeSymbol(symbol) => symbol.clone(),
            CaseFormParam::ValueSymbol(symbol) => symbol.clone(),
            CaseFormParam::App(form) => form.to_string(),
            CaseFormParam::Let(form) => form.to_string(),
        }
    }
}

impl fmt::Display for CaseFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormMatchCase {
    Empty,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
}

impl CaseFormMatchCase {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchCase::Empty => "()".into(),
            CaseFormMatchCase::Prim(prim) => prim.clone(),
            CaseFormMatchCase::TypeKeyword(keyword) => keyword.clone(),
            CaseFormMatchCase::TypeSymbol(symbol) => symbol.clone(),
            CaseFormMatchCase::ValueSymbol(symbol) => symbol.clone(),
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
        CaseFormMatchCase::Empty
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormMatchAction {
    Ignore,
    Panic,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    Fun(Box<FunForm>),
}

impl CaseFormMatchAction {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchAction::Ignore => "_".into(),
            CaseFormMatchAction::Panic => "panic".into(),
            CaseFormMatchAction::Prim(prim) => prim.clone(),
            CaseFormMatchAction::TypeKeyword(keyword) => keyword.clone(),
            CaseFormMatchAction::TypeSymbol(symbol) => symbol.clone(),
            CaseFormMatchAction::ValueSymbol(symbol) => symbol.clone(),
            CaseFormMatchAction::Fun(form) => form.to_string(),
        }
    }
}

impl Default for CaseFormMatchAction {
    fn default() -> CaseFormMatchAction {
        CaseFormMatchAction::Ignore
    }
}

impl fmt::Display for CaseFormMatchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct CaseFormMatch {
    pub tokens: Box<Tokens>,
    pub case: CaseFormMatchCase,
    pub action: CaseFormMatchAction,
}

impl CaseFormMatch {
    pub fn new() -> CaseFormMatch {
        CaseFormMatch::default()
    }

    pub fn from_form(form: &Form) -> Result<CaseFormMatch> {
        if form.name != "match" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a match keyword".into(),
            }));
        }

        if form.params.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a symbol, primitive or application followed by a function".into(),
            }));
        }

        let mut case_match = CaseFormMatch::new();
        case_match.tokens = form.tokens.clone();

        match form.params[0].clone() {
            FormParam::Empty => {
                case_match.case = CaseFormMatchCase::Empty;
            }
            FormParam::Prim(prim) => {
                case_match.case = CaseFormMatchCase::Prim(prim);
            }
            FormParam::TypeKeyword(keyword) => {
                case_match.case = CaseFormMatchCase::TypeKeyword(keyword);
            }
            FormParam::TypeSymbol(symbol) => {
                case_match.case = CaseFormMatchCase::TypeSymbol(symbol);
            }
            FormParam::ValueSymbol(symbol) => {
                case_match.case = CaseFormMatchCase::ValueSymbol(symbol);
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected an empty literal or a primitive or a type keyword or a symbol"
                        .into(),
                }));
            }
        }

        match form.params[1].clone() {
            FormParam::Ignore => {
                case_match.action = CaseFormMatchAction::Ignore;
            }
            FormParam::Prim(prim) => {
                case_match.action = CaseFormMatchAction::Prim(prim);
            }
            FormParam::ValueKeyword(keyword) => {
                if keyword != "panic" {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected the panic keyword".into(),
                    }));
                }

                case_match.action = CaseFormMatchAction::Panic;
            }
            FormParam::TypeKeyword(keyword) => {
                case_match.action = CaseFormMatchAction::TypeKeyword(keyword);
            }
            FormParam::TypeSymbol(symbol) => {
                case_match.action = CaseFormMatchAction::TypeSymbol(symbol);
            }
            FormParam::ValueSymbol(symbol) => {
                case_match.action = CaseFormMatchAction::ValueSymbol(symbol);
            }
            FormParam::Form(form) => {
                if let Ok(form) = FunForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::Fun(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a function form".into(),
                    }));
                }
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected an ignore keyword or a function form".into(),
                }));
            }
        }

        Ok(case_match)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<CaseFormMatch> {
        let form = Form::from_tokens(tokens)?;

        CaseFormMatch::from_form(&form)
    }

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

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct CaseForm {
    pub tokens: Box<Tokens>,
    pub param: CaseFormParam,
    pub matches: Vec<CaseFormMatch>,
}

impl CaseForm {
    pub fn new() -> CaseForm {
        CaseForm::default()
    }

    pub fn matches_to_string(&self) -> String {
        self.matches
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn from_form(form: &Form) -> Result<CaseForm> {
        if form.name != "case" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a case keyword".into(),
            }));
        }

        if form.params.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a case form parameter and at least one match branch".into(),
            }));
        }

        let mut case = CaseForm::new();
        case.tokens = form.tokens.clone();

        match form.params[0].clone() {
            FormParam::Empty => {
                case.param = CaseFormParam::Empty;
            }
            FormParam::Prim(prim) => {
                case.param = CaseFormParam::Prim(prim);
            }
            FormParam::TypeKeyword(keyword) => {
                case.param = CaseFormParam::TypeKeyword(keyword);
            }
            FormParam::TypeSymbol(symbol) => {
                case.param = CaseFormParam::TypeSymbol(symbol);
            }
            FormParam::ValueSymbol(symbol) => {
                case.param = CaseFormParam::ValueSymbol(symbol);
            }
            FormParam::Form(form) => {
                if let Ok(form) = LetForm::from_form(&form) {
                    case.param = CaseFormParam::Let(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    case.param = CaseFormParam::App(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a let form or an application form".into(),
                    }));
                }
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected an empty literal or a primitive or a type keyword or a symbol or an application".into(),
                }));
            }
        }

        for param in form.params[1..].iter() {
            match param.clone() {
                FormParam::Form(form) => {
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
                        desc: "expected a form".into(),
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

    pub fn from_str(s: &str) -> Result<CaseForm> {
        let tokens = Tokens::from_str(s)?;

        CaseForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(case {} {})",
            self.param.to_string(),
            self.matches_to_string(),
        )
    }
}

impl fmt::Display for CaseForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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

        s = "(match True id)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match True (fun t \"True\"))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match 0 (fun n \"0\"))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match Char (fun t \"Char\"))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match '0' (fun c 0))";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match () _)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match T id)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());

        s = "(match E panic)";

        res = CaseFormMatch::from_str(s);

        assert!(res.is_ok());
    }

    #[test]
    fn case_form_from_str() {
        use super::CaseForm;

        let mut s = "(case t (match True (fun t \"True\")) (match False (fun f \"False\")))";

        let mut res = CaseForm::from_str(s);

        assert!(res.is_ok());

        s = "(case (id True) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        s = "(case (let (id True)) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        s = "(case True (match True (fun t \"True\")) (match False _))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        s = "(case res (match T id) (match E panic))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        res.unwrap();
    }
}
