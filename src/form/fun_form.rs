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
    Prim(SimpleValue),
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
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FunFormBody::Empty(_) => "()".into(),
            FunFormBody::Panic(_) => "panic".into(),
            FunFormBody::Prim(prim) => prim.to_string(),
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

    pub fn check_linearly_ordered_on_parameters(&self, parameters: &mut Vec<String>) -> Result<()> {
        match self.body.clone() {
            FunFormBody::Empty(_) | FunFormBody::Panic(_) | FunFormBody::Prim(_) => {}
            FunFormBody::TypeKeyword(symbol) => {
                if parameters.len() != 1 {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!(
                            "non-linear use of parameters {}: {}",
                            parameters.join(", "),
                            symbol
                        ),
                    }));
                }

                parameters.remove(0);
            }
            FunFormBody::ValueSymbol(symbol) => {
                if parameters.len() != 1 {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!(
                            "non-linear use of parameters {}: {}",
                            parameters.join(", "),
                            symbol
                        ),
                    }));
                }

                parameters.remove(0);
            }
            FunFormBody::TypeSymbol(symbol) => {
                if parameters.len() != 1 {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!(
                            "non-linear use of parameters {}: {}",
                            parameters.join(", "),
                            symbol
                        ),
                    }));
                }

                parameters.remove(0);
            }
            FunFormBody::ValuePathSymbol(symbol) => {
                if parameters.len() != 1 {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!(
                            "non-linear use of parameters {}: {}",
                            parameters.join(", "),
                            symbol
                        ),
                    }));
                }

                parameters.remove(0);
            }
            FunFormBody::TypePathSymbol(symbol) => {
                if parameters.len() != 1 {
                    return Err(Error::Semantic(SemanticError {
                        loc: self.loc(),
                        desc: format!(
                            "non-linear use of parameters {}: {}",
                            parameters.join(", "),
                            symbol
                        ),
                    }));
                }

                parameters.remove(0);
            }
            FunFormBody::TypesForm(form) => {
                form.check_linearly_ordered_on_parameters(parameters)?;
            }
            FunFormBody::ProdForm(form) => {
                form.check_linearly_ordered_on_parameters(parameters)?;
            }
            FunFormBody::AppForm(form) => {
                form.check_linearly_ordered_on_parameters(parameters)?;
            }
            FunFormBody::LetForm(form) => {
                form.check_parameters_use()?;
                form.check_linearly_ordered_on_parameters(parameters)?;
            }
            FunFormBody::CaseForm(form) => {
                form.check_parameters_use()?;
                form.check_linearly_ordered_on_parameters(parameters)?;
            }
            FunFormBody::FunForm(form) => {
                form.check_parameters_use()?;
                parameters.extend(
                    form.parameters
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>(),
                );
                form.check_linearly_ordered_on_parameters(parameters)?;
            }
        }

        parameters.clear();

        Ok(())
    }

    pub fn check_parameters_use(&self) -> Result<()> {
        let mut parameters = self
            .parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        self.check_linearly_ordered_on_parameters(&mut parameters)
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
                desc: "expected a symbol or form and a primitive, or a symbol or a form".into(),
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
                SimpleValue::Prim(_) => {
                    fun.body = FunFormBody::Prim(value);
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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_err());

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

        //assert!(form.check_parameters_use().is_err());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_ok());

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

        //assert!(form.check_parameters_use().is_err());

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

        //assert!(form.check_parameters_use().is_err());
    }
}
