use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::arr_form::ArrForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::let_form::LetForm;
use crate::form::list_form::ListForm;
use crate::form::map_form::MapForm;
use crate::form::pair_form::PairForm;
use crate::form::vec_form::VecForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum FunFormParameter {
    Empty(SimpleValue),
    ValueSymbol(SimpleValue),
    MapForm(Box<MapForm>),
    VecForm(Box<VecForm>),
    ArrForm(Box<ArrForm>),
    ListForm(Box<ListForm>),
    PairForm(Box<PairForm>),
}

impl Default for FunFormParameter {
    fn default() -> FunFormParameter {
        FunFormParameter::Empty(SimpleValue::new())
    }
}

impl FunFormParameter {
    pub fn file(&self) -> String {
        match self {
            FunFormParameter::Empty(empty) => empty.file(),
            FunFormParameter::ValueSymbol(symbol) => symbol.file(),
            FunFormParameter::MapForm(form) => form.file(),
            FunFormParameter::VecForm(form) => form.file(),
            FunFormParameter::ArrForm(form) => form.file(),
            FunFormParameter::ListForm(form) => form.file(),
            FunFormParameter::PairForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            FunFormParameter::Empty(empty) => empty.loc(),
            FunFormParameter::ValueSymbol(symbol) => symbol.loc(),
            FunFormParameter::MapForm(form) => form.loc(),
            FunFormParameter::VecForm(form) => form.loc(),
            FunFormParameter::ArrForm(form) => form.loc(),
            FunFormParameter::ListForm(form) => form.loc(),
            FunFormParameter::PairForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormParameter::Empty(_) => "()".into(),
            FunFormParameter::ValueSymbol(symbol) => symbol.to_string(),
            FunFormParameter::MapForm(form) => form.to_string(),
            FunFormParameter::VecForm(form) => form.to_string(),
            FunFormParameter::ArrForm(form) => form.to_string(),
            FunFormParameter::ListForm(form) => form.to_string(),
            FunFormParameter::PairForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for FunFormParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum FunFormBody {
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    MapForm(Box<MapForm>),
    VecForm(Box<VecForm>),
    ArrForm(Box<ArrForm>),
    ListForm(Box<ListForm>),
    PairForm(Box<PairForm>),
    AppForm(Box<AppForm>),
    LetForm(Box<LetForm>),
    CaseForm(Box<CaseForm>),
    FunForm(Box<FunForm>),
}

impl Default for FunFormBody {
    fn default() -> FunFormBody {
        FunFormBody::Empty(SimpleValue::new())
    }
}

impl FunFormBody {
    pub fn file(&self) -> String {
        match self {
            FunFormBody::Empty(empty) => empty.file(),
            FunFormBody::Panic(panic) => panic.file(),
            FunFormBody::Atomic(atomic) => atomic.file(),
            FunFormBody::ValueSymbol(symbol) => symbol.file(),
            FunFormBody::ValuePathSymbol(symbol) => symbol.file(),
            FunFormBody::MapForm(form) => form.file(),
            FunFormBody::VecForm(form) => form.file(),
            FunFormBody::ArrForm(form) => form.file(),
            FunFormBody::ListForm(form) => form.file(),
            FunFormBody::PairForm(form) => form.file(),
            FunFormBody::AppForm(form) => form.file(),
            FunFormBody::LetForm(form) => form.file(),
            FunFormBody::CaseForm(form) => form.file(),
            FunFormBody::FunForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            FunFormBody::Empty(empty) => empty.loc(),
            FunFormBody::Panic(panic) => panic.loc(),
            FunFormBody::Atomic(atomic) => atomic.loc(),
            FunFormBody::ValueSymbol(symbol) => symbol.loc(),
            FunFormBody::ValuePathSymbol(symbol) => symbol.loc(),
            FunFormBody::MapForm(form) => form.loc(),
            FunFormBody::VecForm(form) => form.loc(),
            FunFormBody::ArrForm(form) => form.loc(),
            FunFormBody::ListForm(form) => form.loc(),
            FunFormBody::PairForm(form) => form.loc(),
            FunFormBody::AppForm(form) => form.loc(),
            FunFormBody::LetForm(form) => form.loc(),
            FunFormBody::CaseForm(form) => form.loc(),
            FunFormBody::FunForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormBody::Empty(_) => "()".into(),
            FunFormBody::Panic(_) => "panic".into(),
            FunFormBody::Atomic(atomic) => atomic.to_string(),
            FunFormBody::ValueSymbol(symbol) => symbol.to_string(),
            FunFormBody::ValuePathSymbol(symbol) => symbol.to_string(),
            FunFormBody::MapForm(form) => form.to_string(),
            FunFormBody::VecForm(form) => form.to_string(),
            FunFormBody::ArrForm(form) => form.to_string(),
            FunFormBody::ListForm(form) => form.to_string(),
            FunFormBody::PairForm(form) => form.to_string(),
            FunFormBody::AppForm(form) => form.to_string(),
            FunFormBody::LetForm(form) => form.to_string(),
            FunFormBody::CaseForm(form) => form.to_string(),
            FunFormBody::FunForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for FunFormBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct FunForm {
    pub tokens: Box<Tokens>,
    pub parameters: Vec<FunFormParameter>,
    pub body: FunFormBody,
}

impl FunForm {
    pub fn new() -> FunForm {
        FunForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn parameters_to_string(&self) -> String {
        self.parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for param in self.parameters.iter() {
            match param.clone() {
                FunFormParameter::ValueSymbol(value) => {
                    params.push(value);
                }
                FunFormParameter::MapForm(form) => {
                    params.extend(form.all_variables());
                }
                FunFormParameter::VecForm(form) => {
                    params.extend(form.all_variables());
                }
                FunFormParameter::ArrForm(form) => {
                    params.extend(form.all_variables());
                }
                FunFormParameter::ListForm(form) => {
                    params.extend(form.all_variables());
                }
                FunFormParameter::PairForm(form) => {
                    params.extend(form.all_variables());
                }
                _ => {}
            }
        }

        match self.body.clone() {
            FunFormBody::MapForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::VecForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::ListForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::ArrForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::PairForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.body.clone() {
            FunFormBody::ValueSymbol(value) => {
                vars.push(value);
            }
            FunFormBody::ValuePathSymbol(value) => {
                vars.push(value);
            }
            FunFormBody::MapForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::VecForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::ArrForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::ListForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::PairForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::CaseForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            _ => {}
        }

        vars
    }

    pub fn all_bound_variables(&self) -> Vec<SimpleValue> {
        let all_parameters = self.all_parameters();

        self.all_variables()
            .iter()
            .filter(|v| {
                all_parameters
                    .iter()
                    .any(|p| v.to_string() == p.to_string())
            })
            .map(|v| v.to_owned())
            .collect::<Vec<SimpleValue>>()
    }

    pub fn all_unbound_variables(&self) -> Vec<SimpleValue> {
        let all_parameters = self.all_parameters();

        self.all_variables()
            .iter()
            .filter(|v| {
                !all_parameters
                    .iter()
                    .any(|p| v.to_string() == p.to_string())
            })
            .map(|v| v.to_owned())
            .collect::<Vec<SimpleValue>>()
    }

    pub fn check_parameters_use(&self) -> Result<()> {
        let params = self.all_parameters();
        let params_len = params.len();

        let bound_vars = self.all_bound_variables();
        let bound_vars_len = bound_vars.len();

        if params_len == 0 && bound_vars_len == 0 {
            return Ok(());
        }

        if params_len > bound_vars_len {
            if params_len == 1 && bound_vars_len == 0 {
                return Ok(());
            }

            return Err(Error::Semantic(SemanticError {
                loc: self.loc(),
                desc: "non-linear use of parameters: unused parameters".into(),
            }));
        }

        if params_len < bound_vars_len {
            return Err(Error::Semantic(SemanticError {
                loc: self.loc(),
                desc: "non-linear use of parameters: reused parameters".into(),
            }));
        }

        for (idx, param) in params.iter().enumerate() {
            let bound_var = bound_vars[idx].clone();

            if param.to_string() != bound_var.to_string() {
                return Err(Error::Semantic(SemanticError {
                    loc: bound_var.loc(),
                    desc: format!(
                        "non-ordered use of parameters: expected variable {}",
                        param.to_string()
                    ),
                }));
            }
        }

        Ok(())
    }

    pub fn parse_params(&mut self, form: &Form) -> Result<()> {
        let len = form.tail.len();

        match len {
            2 => match form.tail[0].clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Empty(_) => self.parameters.push(FunFormParameter::Empty(value)),
                    SimpleValue::ValueSymbol(_) => {
                        self.parameters.push(FunFormParameter::ValueSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "expected an unqualified value symbol or an empty literal".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = MapForm::from_form(&form) {
                        if !form.can_be_parameter() {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a symbolic map form".into(),
                            }));
                        }

                        self.parameters
                            .push(FunFormParameter::MapForm(Box::new(form)));
                    } else if let Ok(form) = VecForm::from_form(&form) {
                        if !form.can_be_parameter() {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a symbolic vec form".into(),
                            }));
                        }

                        self.parameters
                            .push(FunFormParameter::VecForm(Box::new(form)));
                    } else if let Ok(form) = ArrForm::from_form(&form) {
                        if !form.can_be_parameter() {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a symbolic arr form".into(),
                            }));
                        }

                        self.parameters
                            .push(FunFormParameter::ArrForm(Box::new(form)));
                    } else if let Ok(form) = ListForm::from_form(&form) {
                        if !form.can_be_parameter() {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a symbolic list form".into(),
                            }));
                        }

                        self.parameters
                            .push(FunFormParameter::ListForm(Box::new(form)));
                    } else if let Ok(form) = PairForm::from_form(&form) {
                        if !form.can_be_parameter() {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a symbolic pair form".into(),
                            }));
                        }

                        self.parameters
                            .push(FunFormParameter::PairForm(Box::new(form)));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            },
            x if x > 2 => {
                for param in form.tail[0..len - 1].iter() {
                    match param.clone() {
                        FormTailElement::Simple(value) => {
                            match value {
                                SimpleValue::ValueSymbol(_) => {
                                    self.parameters.push(FunFormParameter::ValueSymbol(value));
                                }
                                x => {
                                    return Err(Error::Syntactic(SyntacticError {
                                    loc: x.loc(),
                                    desc: "expected an unqualified value symbol or an empty literal".into(),
                                }));
                                }
                            }
                        }
                        FormTailElement::Form(form) => {
                            if let Ok(form) = MapForm::from_form(&form) {
                                if !form.can_be_parameter() {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected a symbolic map form".into(),
                                    }));
                                }

                                self.parameters
                                    .push(FunFormParameter::MapForm(Box::new(form)));
                            } else if let Ok(form) = VecForm::from_form(&form) {
                                if !form.can_be_parameter() {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected a symbolic vec form".into(),
                                    }));
                                }

                                self.parameters
                                    .push(FunFormParameter::VecForm(Box::new(form)));
                            } else if let Ok(form) = ArrForm::from_form(&form) {
                                if !form.can_be_parameter() {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected a symbolic arr form".into(),
                                    }));
                                }

                                self.parameters
                                    .push(FunFormParameter::ArrForm(Box::new(form)));
                            } else if let Ok(form) = ListForm::from_form(&form) {
                                if !form.can_be_parameter() {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected a symbolic list form".into(),
                                    }));
                                }

                                self.parameters
                                    .push(FunFormParameter::ListForm(Box::new(form)));
                            } else if let Ok(form) = PairForm::from_form(&form) {
                                if !form.can_be_parameter() {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected a symbolic pair form".into(),
                                    }));
                                }

                                self.parameters
                                    .push(FunFormParameter::PairForm(Box::new(form)));
                            } else {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "unexpected form".into(),
                                }));
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected at least a parameter and a function body".into(),
                }));
            }
        }

        Ok(())
    }

    pub fn parse_body(&mut self, form: &Form) -> Result<()> {
        match form.tail[form.tail.len() - 1].clone() {
            FormTailElement::Simple(value) => match value.clone() {
                SimpleValue::Empty(_) => {
                    self.body = FunFormBody::Empty(value);
                }
                SimpleValue::Atomic(_) => {
                    self.body = FunFormBody::Atomic(value);
                }
                SimpleValue::Panic(_) => {
                    self.body = FunFormBody::Panic(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    self.body = FunFormBody::ValueSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    self.body = FunFormBody::ValuePathSymbol(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "unexpected function body".into(),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(form) = PairForm::from_form(&form) {
                    self.body = FunFormBody::PairForm(Box::new(form));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    self.body = FunFormBody::LetForm(Box::new(form));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    self.body = FunFormBody::CaseForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    self.body = FunFormBody::FunForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    self.body = FunFormBody::AppForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected form".into(),
                    }));
                }
            }
        }

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<FunForm> {
        if form.head.to_string() != "fun" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a fun keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a parameter and a function body".into(),
            }));
        }

        let mut fun = FunForm::new();
        fun.tokens = form.tokens.clone();

        fun.parse_params(&form)?;

        fun.parse_body(&form)?;

        Ok(fun)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<FunForm> {
        let form = Form::from_tokens(tokens)?;

        FunForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<FunForm> {
        let tokens = Tokens::from_str(s)?;

        FunForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(fun {} {})",
            self.parameters_to_string(),
            self.body.to_string(),
        )
    }
}

impl fmt::Display for FunForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for FunForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fun_form_from_str() {
        use super::FunForm;

        let mut s = "(fun () x)";

        let mut res = FunForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "()".to_string());
        assert_eq!(form.body.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun x ())";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "x".to_string());
        assert_eq!(form.body.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun x moduleX.x)";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "x".to_string());
        assert_eq!(form.body.to_string(), "moduleX.x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun a (pair b c) (math.+ a b c))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "a (pair b c)".to_string());
        assert_eq!(form.body.to_string(), "(math.+ a b c)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun a (list b c) (math.+ a b c))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "a (list b c)".to_string());
        assert_eq!(form.body.to_string(), "(math.+ a b c)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun (map (pair a b) (pair c d)) (math.+ a b c d))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.parameters_to_string(),
            "(map (pair a b) (pair c d))".to_string()
        );
        assert_eq!(form.body.to_string(), "(math.+ a b c d)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun a b c d (math.+ a b 10 (math.* c d 10)))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "a b c d".to_string());
        assert_eq!(
            form.body.to_string(),
            "(math.+ a b 10 (math.* c d 10))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun a b c d (fun e (math.+ a b 10 (math.* c d e))))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "a b c d".to_string());
        assert_eq!(
            form.body.to_string(),
            "(fun e (math.+ a b 10 (math.* c d e)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }

    #[test]
    fn fun_form_variables_and_parameters() {
        use super::FunForm;

        let mut s = "(fun () ())";

        let mut form = FunForm::from_str(s).unwrap();

        let mut all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "".to_string());

        let mut all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "".to_string());

        let mut all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "".to_string());

        let mut all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a ())";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a 'a')";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a a)";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a b)";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "b".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "b".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a (+ a 1))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "+, a".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "+".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a b c d (+ a b c d 1))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, b, c, d".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "+, a, b, c, d".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, b, c, d".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "+".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a b d c (+ a b c d 1))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, b, d, c".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "+, a, b, c, d".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, b, c, d".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "+".to_string());

        assert!(form.check_parameters_use().is_err());

        s = "(fun a b c d e (+ a b c d 1))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, b, c, d, e".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "+, a, b, c, d".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, b, c, d".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "+".to_string());

        assert!(form.check_parameters_use().is_err());

        s = "(fun a b c d e (+ a b c d e f))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, b, c, d, e".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "+, a, b, c, d, e, f".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, b, c, d, e".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "+, f".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a (case a (match T 'T') (match F 'F')))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a (case a (match T id) (match F (fun bool (printBool bool)))))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, bool".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a, printBool, bool".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, bool".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "printBool".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "
            (fun a (case a
                (match T id)
                (match F (fun bool (let
                    (val f (fun () (printBool bool)))
                    (f ()))))))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, bool, f".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a, printBool, bool, f".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, bool, f".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "printBool".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "(fun a b (case a
                (match T id)
                (match F (fun bool (let
                    (val f (fun () (printBool bool)))
                    (f ()))))))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, b, bool, f".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a, printBool, bool, f".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, bool, f".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "printBool".to_string());

        assert!(form.check_parameters_use().is_err());

        s = "(fun b a (case a
                (match T id)
                (match F (fun bool (let
                    (val f (fun () (printBool bool)))
                    (f ()))))))";

        form = FunForm::from_str(s).unwrap();

        all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "b, a, bool, f".to_string());

        all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a, printBool, bool, f".to_string());

        all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, bool, f".to_string());

        all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "printBool".to_string());

        assert!(form.check_parameters_use().is_err());
    }
}
