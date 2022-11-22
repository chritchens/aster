use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::form::{Form, FormParam};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::ProdForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormParam {
    Empty,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    AppForm(Box<AppForm>),
    LetForm(Box<LetForm>),
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
            CaseFormParam::AppForm(form) => form.to_string(),
            CaseFormParam::LetForm(form) => form.to_string(),
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
    Prim(String),
    ValueKeyword(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
}

impl CaseFormMatchAction {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchAction::Ignore => "_".into(),
            CaseFormMatchAction::Prim(prim) => prim.clone(),
            CaseFormMatchAction::ValueKeyword(keyword) => keyword.clone(),
            CaseFormMatchAction::TypeKeyword(keyword) => keyword.clone(),
            CaseFormMatchAction::TypeSymbol(symbol) => symbol.clone(),
            CaseFormMatchAction::ValueSymbol(symbol) => symbol.clone(),
            CaseFormMatchAction::ProdForm(form) => form.to_string(),
            CaseFormMatchAction::FunForm(form) => form.to_string(),
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

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
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
                case_match.action = CaseFormMatchAction::ValueKeyword(keyword);
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
                if let Ok(form) = ProdForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::ProdForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::FunForm(Box::new(form));
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

    pub fn check_linearly_ordered_on_params(&self, params: &mut Vec<String>) -> Result<()> {
        match self.param.clone() {
            CaseFormParam::TypeSymbol(symbol) => {
                if params[0] != symbol {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!("non-linear use of params {}: {}", params.join(", "), symbol),
                    }));
                }

                params.remove(0);
            }
            CaseFormParam::ValueSymbol(symbol) => {
                if params[0] != symbol {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!("non-linear use of params {}: {}", params.join(", "), symbol),
                    }));
                }

                params.remove(0);
            }
            CaseFormParam::AppForm(form) => {
                form.check_linearly_ordered_on_params(params)?;
            }
            CaseFormParam::LetForm(form) => {
                form.check_params_use()?;
                form.check_linearly_ordered_on_params(params)?;
            }
            _ => {}
        }

        for form in self.matches.iter() {
            match form.action.clone() {
                CaseFormMatchAction::ProdForm(form) => {
                    form.check_linearly_ordered_on_params(params)?;
                }
                CaseFormMatchAction::FunForm(form) => {
                    form.check_params_use()?;
                    params.extend(
                        form.params
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<String>>()
                            .clone(),
                    );
                    form.check_linearly_ordered_on_params(params)?;
                }
                _ => {}
            }
        }

        params.clear();

        Ok(())
    }

    pub fn check_params_use(&self) -> Result<()> {
        for form in self.matches.iter() {
            if let CaseFormMatchAction::FunForm(form) = form.action.clone() {
                form.check_params_use()?;
            }
        }

        Ok(())
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
                    case.param = CaseFormParam::LetForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    case.param = CaseFormParam::AppForm(Box::new(form));
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

    #[allow(clippy::should_implement_trait)]
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
    }

    #[test]
    fn case_form_from_str() {
        use super::CaseForm;

        let mut s = "(case t (match True (fun t \"True\")) (match False (fun f \"False\")))";

        let mut res = CaseForm::from_str(s);

        assert!(res.is_ok());

        let mut case = res.unwrap();

        assert_eq!(case.param.to_string(), "t".to_string());
        assert_eq!(
            case.matches_to_string(),
            "(match True (fun t \"True\")) (match False (fun f \"False\"))".to_string()
        );
        assert_eq!(case.to_string(), s.to_string());

        s = "(case (id True) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.param.to_string(), "(id True)".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case (let (id True)) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.param.to_string(), "(let (id True))".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case True (match True (fun t \"True\")) (match False _))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.param.to_string(), "True".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case res (match T id) (match E panic))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.param.to_string(), "res".to_string());
        assert_eq!(
            case.matches_to_string(),
            "(match T id) (match E panic)".to_string()
        );
        assert_eq!(case.to_string(), s.to_string());
    }
}
