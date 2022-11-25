use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::prod_form::ProdForm;
use crate::form::simple_value::SimpleValue;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormVariable {
    Empty(SimpleValue),
    Prim(SimpleValue),
    TypeKeyword(SimpleValue),
    TypeSymbol(SimpleValue),
    ValueSymbol(SimpleValue),
    AppForm(Box<AppForm>),
    LetForm(Box<LetForm>),
}

impl Default for CaseFormVariable {
    fn default() -> CaseFormVariable {
        CaseFormVariable::Empty(SimpleValue::new())
    }
}

impl CaseFormVariable {
    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.clone() {
            CaseFormVariable::Empty(value) => {
                vars.push(value);
            }
            CaseFormVariable::Prim(value) => {
                vars.push(value);
            }
            CaseFormVariable::TypeKeyword(value) => {
                vars.push(value);
            }
            CaseFormVariable::TypeSymbol(value) => {
                vars.push(value);
            }
            CaseFormVariable::ValueSymbol(value) => {
                vars.push(value);
            }
            CaseFormVariable::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormVariable::LetForm(form) => {
                vars.extend(form.all_variables());
            }
        }

        vars
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormVariable::Empty(_) => "()".into(),
            CaseFormVariable::Prim(prim) => prim.to_string(),
            CaseFormVariable::TypeKeyword(keyword) => keyword.to_string(),
            CaseFormVariable::TypeSymbol(symbol) => symbol.to_string(),
            CaseFormVariable::ValueSymbol(symbol) => symbol.to_string(),
            CaseFormVariable::AppForm(form) => form.to_string(),
            CaseFormVariable::LetForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for CaseFormVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormMatchCase {
    Empty(SimpleValue),
    Prim(SimpleValue),
    TypeKeyword(SimpleValue),
    TypeSymbol(SimpleValue),
    ValueSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
}

impl CaseFormMatchCase {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchCase::Empty(_) => "()".into(),
            CaseFormMatchCase::Prim(prim) => prim.to_string(),
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CaseFormMatchAction {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Panic(SimpleValue),
    Prim(SimpleValue),
    ValueKeyword(SimpleValue),
    TypeKeyword(SimpleValue),
    TypeSymbol(SimpleValue),
    ValueSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    ProdForm(Box<ProdForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
}

impl CaseFormMatchAction {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            CaseFormMatchAction::Ignore(_) => "_".into(),
            CaseFormMatchAction::Empty(_) => "_".into(),
            CaseFormMatchAction::Panic(_) => "panic".into(),
            CaseFormMatchAction::Prim(prim) => prim.to_string(),
            CaseFormMatchAction::ValueKeyword(keyword) => keyword.to_string(),
            CaseFormMatchAction::TypeKeyword(keyword) => keyword.to_string(),
            CaseFormMatchAction::TypeSymbol(symbol) => symbol.to_string(),
            CaseFormMatchAction::ValueSymbol(symbol) => symbol.to_string(),
            CaseFormMatchAction::TypePathSymbol(symbol) => symbol.to_string(),
            CaseFormMatchAction::ValuePathSymbol(symbol) => symbol.to_string(),
            CaseFormMatchAction::ProdForm(form) => form.to_string(),
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

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.action.clone() {
            CaseFormMatchAction::Ignore(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::Empty(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::Panic(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::Prim(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::ValueKeyword(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::TypeKeyword(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::TypeSymbol(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::ValueSymbol(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::TypePathSymbol(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::ValuePathSymbol(value) => {
                vars.push(value);
            }
            CaseFormMatchAction::ProdForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormMatchAction::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            CaseFormMatchAction::LetForm(form) => {
                vars.extend(form.all_variables());
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<CaseFormMatch> {
        if form.head.to_string() != "match" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a match keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a symbol, primitive or application followed by a function".into(),
            }));
        }

        let mut case_match = CaseFormMatch::new();
        case_match.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    case_match.case = CaseFormMatchCase::Empty(value);
                }
                SimpleValue::Prim(_) => {
                    case_match.case = CaseFormMatchCase::Prim(value);
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
                        loc: form.loc(),
                        desc: format!("unexpected value: {}", x),
                    }));
                }
            },
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
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
                SimpleValue::Prim(_) => {
                    case_match.action = CaseFormMatchAction::Prim(value);
                }
                SimpleValue::ValueKeyword(_) => {
                    case_match.action = CaseFormMatchAction::ValueKeyword(value);
                }
                SimpleValue::TypeKeyword(_) => {
                    case_match.action = CaseFormMatchAction::TypeKeyword(value);
                }
                SimpleValue::TypeSymbol(_) => {
                    case_match.action = CaseFormMatchAction::TypeSymbol(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    case_match.action = CaseFormMatchAction::ValueSymbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    case_match.action = CaseFormMatchAction::TypePathSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    case_match.action = CaseFormMatchAction::ValuePathSymbol(value);
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(form) = ProdForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::ProdForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::FunForm(Box::new(form));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    case_match.action = CaseFormMatchAction::LetForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a product, case, let or function form".into(),
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

#[derive(Debug, Eq, PartialEq, Clone, Default)]
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

    pub fn check_linearly_ordered_on_params(&self, params: &mut Vec<String>) -> Result<()> {
        match self.variable.clone() {
            CaseFormVariable::TypeSymbol(symbol) => {
                if params[0] != symbol.to_string() {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!("non-linear use of params {}: {}", params.join(", "), symbol),
                    }));
                }

                params.remove(0);
            }
            CaseFormVariable::ValueSymbol(symbol) => {
                if params[0] != symbol.to_string() {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!("non-linear use of params {}: {}", params.join(", "), symbol),
                    }));
                }

                params.remove(0);
            }
            CaseFormVariable::AppForm(form) => {
                form.check_linearly_ordered_on_params(params)?;
            }
            CaseFormVariable::LetForm(form) => {
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
                CaseFormMatchAction::LetForm(form) => {
                    form.check_params_use()?;
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
        if form.head.to_string() != "case" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
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
                SimpleValue::Prim(_) => {
                    case.variable = CaseFormVariable::Prim(value);
                }
                SimpleValue::TypeKeyword(_) => {
                    case.variable = CaseFormVariable::TypeKeyword(value);
                }
                SimpleValue::TypeSymbol(_) => {
                    case.variable = CaseFormVariable::TypeSymbol(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    case.variable = CaseFormVariable::ValueSymbol(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected value: {}", x),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(form) = LetForm::from_form(&form) {
                    case.variable = CaseFormVariable::LetForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    case.variable = CaseFormVariable::AppForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a let form or an application form".into(),
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

        s = "(case (id True) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "(id True)".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case (let (id True)) (match True (fun t \"True\")) (match False (fun f \"False\")))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "(let (id True))".to_string());
        assert_eq!(case.to_string(), s.to_string());

        s = "(case True (match True (fun t \"True\")) (match False _))";

        res = CaseForm::from_str(s);

        assert!(res.is_ok());

        case = res.unwrap();

        assert_eq!(case.variable.to_string(), "True".to_string());
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
