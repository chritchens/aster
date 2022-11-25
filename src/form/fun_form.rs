use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::app_form::AppForm;
use crate::form::case_form::CaseForm;
use crate::form::form::{Form, FormTailElement};
use crate::form::let_form::LetForm;
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::simple_value::SimpleValue;
use crate::form::types_form::TypesForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunFormParameter {
    Empty(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
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
            FunFormParameter::TypeSymbol(symbol) => symbol.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            FunFormParameter::Empty(empty) => empty.loc(),
            FunFormParameter::ValueSymbol(symbol) => symbol.loc(),
            FunFormParameter::TypeSymbol(symbol) => symbol.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormParameter::Empty(_) => "()".into(),
            FunFormParameter::ValueSymbol(symbol) => symbol.to_string(),
            FunFormParameter::TypeSymbol(symbol) => symbol.to_string(),
        }
    }
}

impl fmt::Display for FunFormParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FunFormBody {
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    TypeKeyword(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    TypesForm(Box<TypesForm>),
    ProdForm(Box<ProdForm>),
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
            FunFormBody::TypeKeyword(keyword) => keyword.file(),
            FunFormBody::ValueSymbol(symbol) => symbol.file(),
            FunFormBody::TypeSymbol(symbol) => symbol.file(),
            FunFormBody::ValuePathSymbol(symbol) => symbol.file(),
            FunFormBody::TypePathSymbol(symbol) => symbol.file(),
            FunFormBody::TypesForm(form) => form.file(),
            FunFormBody::ProdForm(form) => form.file(),
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
            FunFormBody::TypeKeyword(keyword) => keyword.loc(),
            FunFormBody::ValueSymbol(symbol) => symbol.loc(),
            FunFormBody::TypeSymbol(symbol) => symbol.loc(),
            FunFormBody::ValuePathSymbol(symbol) => symbol.loc(),
            FunFormBody::TypePathSymbol(symbol) => symbol.loc(),
            FunFormBody::TypesForm(form) => form.loc(),
            FunFormBody::ProdForm(form) => form.loc(),
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
            FunFormBody::TypeKeyword(keyword) => keyword.to_string(),
            FunFormBody::ValueSymbol(symbol) => symbol.to_string(),
            FunFormBody::TypeSymbol(symbol) => symbol.to_string(),
            FunFormBody::ValuePathSymbol(symbol) => symbol.to_string(),
            FunFormBody::TypePathSymbol(symbol) => symbol.to_string(),
            FunFormBody::TypesForm(form) => form.to_string(),
            FunFormBody::ProdForm(form) => form.to_string(),
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

#[derive(Debug, Eq, PartialEq, Clone, Default)]
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
        match self.parameters.len() {
            1 => self.parameters[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.parameters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "()".to_string(),
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for param in self.parameters.iter() {
            match param.clone() {
                FunFormParameter::ValueSymbol(value) => {
                    params.push(value);
                }
                FunFormParameter::TypeSymbol(value) => {
                    params.push(value);
                }
                _ => {}
            }
        }

        match self.body.clone() {
            FunFormBody::TypesForm(form) => {
                params.extend(form.all_parameters());
            }
            FunFormBody::ProdForm(form) => {
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
            FunFormBody::TypeSymbol(value) => {
                vars.push(value);
            }
            FunFormBody::ValuePathSymbol(value) => {
                vars.push(value);
            }
            FunFormBody::TypePathSymbol(value) => {
                vars.push(value);
            }
            FunFormBody::TypesForm(form) => {
                vars.extend(form.all_variables());
            }
            FunFormBody::ProdForm(form) => {
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

    pub fn from_form(form: &Form) -> Result<FunForm> {
        if form.head.to_string() != "fun" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a fun keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a symbol or form and an atomic, or a symbol or a form".into(),
            }));
        }

        let mut fun = FunForm::new();
        fun.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => fun.parameters.push(FunFormParameter::Empty(value)),
                SimpleValue::ValueSymbol(_) => {
                    fun.parameters.push(FunFormParameter::ValueSymbol(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    fun.parameters.push(FunFormParameter::TypeSymbol(value));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected function param: {}", x.to_string()),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::TypeSymbol(symbol) => {
                                fun.parameters.push(FunFormParameter::TypeSymbol(symbol));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                fun.parameters.push(FunFormParameter::ValueSymbol(symbol));
                            }
                            _ => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected a product of unqualified symbols".into(),
                                }));
                            }
                        }
                    }
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a product of symbols".into(),
                    }));
                }
            }
        }

        match form.tail[1].clone() {
            FormTailElement::Simple(value) => match value.clone() {
                SimpleValue::Empty(_) => {
                    fun.body = FunFormBody::Empty(value);
                }
                SimpleValue::Atomic(_) => {
                    fun.body = FunFormBody::Atomic(value);
                }
                SimpleValue::Panic(_) => {
                    fun.body = FunFormBody::Panic(value);
                }
                SimpleValue::TypeKeyword(_) => {
                    fun.body = FunFormBody::TypeKeyword(value);
                }
                SimpleValue::ValueSymbol(_) => {
                    fun.body = FunFormBody::ValueSymbol(value);
                }
                SimpleValue::TypeSymbol(_) => {
                    fun.body = FunFormBody::TypeSymbol(value);
                }
                SimpleValue::ValuePathSymbol(_) => {
                    fun.body = FunFormBody::ValuePathSymbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    fun.body = FunFormBody::TypePathSymbol(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected function body: {}", x.to_string()),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(form) = TypesForm::from_form(&form) {
                    fun.body = FunFormBody::TypesForm(Box::new(form));
                } else if let Ok(form) = ProdForm::from_form(&form) {
                    fun.body = FunFormBody::ProdForm(Box::new(form));
                } else if let Ok(form) = LetForm::from_form(&form) {
                    fun.body = FunFormBody::LetForm(Box::new(form));
                } else if let Ok(form) = CaseForm::from_form(&form) {
                    fun.body = FunFormBody::CaseForm(Box::new(form));
                } else if let Ok(form) = FunForm::from_form(&form) {
                    fun.body = FunFormBody::FunForm(Box::new(form));
                } else if let Ok(form) = AppForm::from_form(&form) {
                    fun.body = FunFormBody::AppForm(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a type form, a let form or an application form".into(),
                    }));
                }
            }
        }

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

        s = "(fun (prod a b c d) (math.+ (prod a b 10 (math.* (prod c d 10)))))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "(prod a b c d)".to_string());
        assert_eq!(
            form.body.to_string(),
            "(math.+ (prod a b 10 (math.* (prod c d 10))))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun (prod a b c d) (fun e (math.+ (prod a b 10 (math.* (prod c d e))))))";

        res = FunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.parameters_to_string(), "(prod a b c d)".to_string());
        assert_eq!(
            form.body.to_string(),
            "(fun e (math.+ (prod a b 10 (math.* (prod c d e)))))".to_string()
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

        s = "(fun a (+ (prod a 1)))";

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

        s = "(fun (prod a b c d) (+ (prod a b c d 1)))";

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

        s = "(fun (prod a b d c) (+ (prod a b c d 1)))";

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

        s = "(fun (prod a b c d e) (+ (prod a b c d 1)))";

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

        s = "(fun (prod a b c d e) (+ (prod a b c d e f)))";

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

        s = "(fun (prod a b) (case a
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

        s = "(fun (prod b a) (case a
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
