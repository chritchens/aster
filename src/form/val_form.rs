use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::fun_form::FunForm;
use crate::form::let_form::LetForm;
use crate::form::pair_form::PairForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_value_symbol;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum ValFormValue {
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueSymbol(SimpleValue),
    PairForm(Box<PairForm>),
    FunForm(Box<FunForm>),
    LetForm(Box<LetForm>),
    AppForm(Box<AppForm>),
    CaseForm(Box<CaseForm>),
}

impl Default for ValFormValue {
    fn default() -> ValFormValue {
        ValFormValue::Empty(SimpleValue::new())
    }
}

impl ValFormValue {
    pub fn file(&self) -> String {
        match self {
            ValFormValue::Empty(empty) => empty.file(),
            ValFormValue::Panic(panic) => panic.file(),
            ValFormValue::Atomic(atomic) => atomic.file(),
            ValFormValue::ValueSymbol(symbol) => symbol.file(),
            ValFormValue::PairForm(form) => form.file(),
            ValFormValue::FunForm(form) => form.file(),
            ValFormValue::LetForm(form) => form.file(),
            ValFormValue::AppForm(form) => form.file(),
            ValFormValue::CaseForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ValFormValue::Empty(empty) => empty.loc(),
            ValFormValue::Panic(panic) => panic.loc(),
            ValFormValue::Atomic(atomic) => atomic.loc(),
            ValFormValue::ValueSymbol(symbol) => symbol.loc(),
            ValFormValue::PairForm(form) => form.loc(),
            ValFormValue::FunForm(form) => form.loc(),
            ValFormValue::LetForm(form) => form.loc(),
            ValFormValue::AppForm(form) => form.loc(),
            ValFormValue::CaseForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ValFormValue::Empty(_) => "()".into(),
            ValFormValue::Panic(_) => "panic".into(),
            ValFormValue::Atomic(atomic) => atomic.to_string(),
            ValFormValue::ValueSymbol(symbol) => symbol.to_string(),
            ValFormValue::PairForm(form) => form.to_string(),
            ValFormValue::FunForm(form) => form.to_string(),
            ValFormValue::LetForm(form) => form.to_string(),
            ValFormValue::AppForm(form) => form.to_string(),
            ValFormValue::CaseForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ValFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ValForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub value: ValFormValue,
}

impl ValForm {
    pub fn new() -> ValForm {
        ValForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn is_empty_literal(&self) -> bool {
        match self.value {
            ValFormValue::Empty(_) => true,
            _ => false,
        }
    }

    pub fn is_panic(&self) -> bool {
        match self.value {
            ValFormValue::Panic(_) => true,
            _ => false,
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self.value {
            ValFormValue::Atomic(_) => true,
            _ => false,
        }
    }

    pub fn is_value_symbol(&self) -> bool {
        match self.value {
            ValFormValue::ValueSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_pair_form(&self) -> bool {
        match self.value {
            ValFormValue::PairForm(_) => true,
            _ => false,
        }
    }

    pub fn is_function_form(&self) -> bool {
        match self.value {
            ValFormValue::FunForm(_) => true,
            _ => false,
        }
    }

    pub fn is_application_form(&self) -> bool {
        match self.value {
            ValFormValue::AppForm(_) => true,
            _ => false,
        }
    }

    pub fn is_let_form(&self) -> bool {
        match self.value {
            ValFormValue::LetForm(_) => true,
            _ => false,
        }
    }

    pub fn is_case_form(&self) -> bool {
        match self.value {
            ValFormValue::CaseForm(_) => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        self.is_empty_literal()
            || self.is_atomic()
            || self.is_value_symbol()
            || self.is_pair_form()
            || self.is_function_form()
            || self.is_case_form()
            || (self.is_let_form() && is_value_symbol(&self.name.to_string()))
            || (self.is_application_form() && is_value_symbol(&self.name.to_string()))
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        match self.value.clone() {
            ValFormValue::PairForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::FunForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::LetForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::AppForm(form) => {
                params.extend(form.all_parameters());
            }
            ValFormValue::CaseForm(form) => {
                params.extend(form.all_parameters());
            }
            _ => {}
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self.value.clone() {
            ValFormValue::ValueSymbol(value) => {
                vars.push(value);
            }
            ValFormValue::PairForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::FunForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::LetForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::AppForm(form) => {
                vars.extend(form.all_variables());
            }
            ValFormValue::CaseForm(form) => {
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

    pub fn from_form(form: &Form) -> Result<ValForm> {
        if form.head.to_string() != "val" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a val keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and a value".into(),
            }));
        }

        let mut val = ValForm::new();
        val.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    val.name = value;
                }
                SimpleValue::TypeSymbol(_) => {
                    val.name = value;
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected an unqualified symbol".into(),
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
                SimpleValue::Empty(empty) => {
                    val.value = ValFormValue::Empty(SimpleValue::Empty(empty));
                }
                SimpleValue::Panic(empty) => {
                    val.value = ValFormValue::Panic(SimpleValue::Panic(empty));
                }
                SimpleValue::Atomic(atomic) => {
                    val.value = ValFormValue::Atomic(SimpleValue::Atomic(atomic));
                }
                SimpleValue::ValueSymbol(symbol) => {
                    val.value = ValFormValue::ValueSymbol(SimpleValue::ValueSymbol(symbol));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },

            FormTailElement::Form(form) => match form.head.to_string().as_str() {
                "pair" => {
                    let form = PairForm::from_form(&form)?;
                    val.value = ValFormValue::PairForm(Box::new(form));
                }
                "fun" => {
                    let form = FunForm::from_form(&form)?;
                    val.value = ValFormValue::FunForm(Box::new(form));
                }
                "let" => {
                    let form = LetForm::from_form(&form)?;
                    val.value = ValFormValue::LetForm(Box::new(form));
                }
                "case" => {
                    let form = CaseForm::from_form(&form)?;
                    val.value = ValFormValue::CaseForm(Box::new(form));
                }
                _ => {
                    if let Ok(form) = AppForm::from_form(&form) {
                        val.value = ValFormValue::AppForm(Box::new(form));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "unexpected form".into(),
                        }));
                    }
                }
            },
        }

        Ok(val)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ValForm> {
        let form = Form::from_tokens(tokens)?;

        ValForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ValForm> {
        let tokens = Tokens::from_str(s)?;

        ValForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(val {} {})", self.name, self.value.to_string())
    }
}

impl fmt::Display for ValForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for ValForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn val_form_from_str() {
        use super::ValForm;

        let mut s = "(val empty ())";

        let mut res = ValForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "empty".to_string());
        assert_eq!(form.value.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_empty_literal());
        assert!(form.is_value());

        s = "(val x 10)";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_atomic());
        assert!(form.is_value());

        s = "(val w x)";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "w".to_string());
        assert_eq!(form.value.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_value_symbol());
        assert!(form.is_value());

        s = "(val s (math.+ 10.323 1))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "s".to_string());
        assert_eq!(form.value.to_string(), "(math.+ 10.323 1)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_application_form());
        assert!(form.is_value());

        s = "(val p (pair a (pair b c)))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "p".to_string());
        assert_eq!(form.value.to_string(), "(pair a (pair b c))".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_pair_form());
        assert!(form.is_value());

        s = "(val p (pair a (pair (f x y 10) 11)))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "p".to_string());
        assert_eq!(
            form.value.to_string(),
            "(pair a (pair (f x y 10) 11))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_pair_form());
        assert!(form.is_value());

        s = "(val err (let (type StringError String) (unwrap \"error\")))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "err".to_string());
        assert_eq!(
            form.value.to_string(),
            "(let (type StringError String) (unwrap \"error\"))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_let_form());
        assert!(form.is_value());

        s = "(val unwrap (fun res (case res (match T id) (match E panic))))";

        res = ValForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "unwrap".to_string());
        assert_eq!(
            form.value.to_string(),
            "(fun res (case res (match T id) (match E panic)))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_function_form());
        assert!(form.is_value());
    }

    #[test]
    fn val_form_check_parameters_use() {
        use super::ValForm;

        let mut s = "(val x (fun a (case a (match T id) (match F (fun bool (printBool bool))))))";

        let mut form = ValForm::from_str(s).unwrap();

        let mut all_parameters = form
            .all_parameters()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_parameters, "a, bool".to_string());

        let mut all_variables = form
            .all_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_variables, "a, printBool, bool".to_string());

        let mut all_bound_variables = form
            .all_bound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_bound_variables, "a, bool".to_string());

        let mut all_unbound_variables = form
            .all_unbound_variables()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        assert_eq!(all_unbound_variables, "printBool".to_string());

        assert!(form.check_parameters_use().is_ok());

        s = "
            (val x (fun a (case a
                (match T id)
                (match F (fun bool (let
                    (val f (fun () (printBool bool)))
                    (f ())))))))";

        form = ValForm::from_str(s).unwrap();

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

        s = "(val x (fun a b (case a
                (match T id)
                (match F (fun bool (let
                    (val f (fun () (printBool bool)))
                    (f ())))))))";

        form = ValForm::from_str(s).unwrap();

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

        s = "(val x (fun b a (case a
                (match T id)
                (match F (fun bool (let
                    (val f (fun () (printBool bool)))
                    (f ())))))))";

        form = ValForm::from_str(s).unwrap();

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
