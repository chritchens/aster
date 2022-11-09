use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_qualified, is_value_symbol, symbol_name};
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use crate::value::forms::prod_form::{ProdForm, ProdFormValue};
use crate::value::forms::type_form::TypeForm;
use crate::value::forms::value_form::ValueForm;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AppFormTypeParam {
    Ignore,
    Keyword(String),
    Symbol(String),
    Form(TypeForm),
}

impl Default for AppFormTypeParam {
    fn default() -> AppFormTypeParam {
        AppFormTypeParam::Ignore
    }
}

impl AppFormTypeParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormTypeParam::Ignore => "_".into(),
            AppFormTypeParam::Keyword(keyword) => keyword.clone(),
            AppFormTypeParam::Symbol(symbol) => symbol.clone(),
            AppFormTypeParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AppFormTypeParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AppFormParam {
    Empty,
    Prim(String),
    TypeKeyword(String),
    TypeSymbol(String),
    ValueSymbol(String),
    TypeForm(TypeForm),
    ValueForm(ValueForm),
    MixedForm(Form),
}

impl Default for AppFormParam {
    fn default() -> AppFormParam {
        AppFormParam::Empty
    }
}

impl AppFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AppFormParam::Empty => "()".into(),
            AppFormParam::Prim(prim) => prim.clone(),
            AppFormParam::TypeKeyword(keyword) => keyword.clone(),
            AppFormParam::TypeSymbol(symbol) => symbol.clone(),
            AppFormParam::ValueSymbol(symbol) => symbol.clone(),
            AppFormParam::TypeForm(form) => form.to_string(),
            AppFormParam::ValueForm(form) => form.to_string(),
            AppFormParam::MixedForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AppFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AppForm {
    pub tokens: Tokens,
    pub name: String,
    pub type_params: Vec<AppFormTypeParam>,
    pub params: Vec<AppFormParam>,
}

impl AppForm {
    pub fn new() -> AppForm {
        AppForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn type_params_to_string(&self) -> String {
        let len = self.type_params.len();

        if len > 1 {
            format!(
                "(prod {})",
                self.type_params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else if len == 1 {
            self.type_params[0].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn params_to_string(&self) -> String {
        let len = self.params.len();

        if len > 1 {
            format!(
                "(prod {})",
                self.params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else if len == 1 {
            self.params[0].to_string()
        } else {
            "()".to_string()
        }
    }

    pub fn from_form(form: &Form) -> Result<AppForm> {
        if !is_value_symbol(&symbol_name(&form.name)) || is_keyword(&symbol_name(&form.name)) {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected a value symbol".into(),
            }));
        }

        let len = form.params.len();

        if form.params.is_empty() {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected at least a parameter or product of parameters".into(),
            }));
        }

        if form.params.len() > 2 {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected a type symbol or product of type symbols and a parameter or product of parameters".into(),
            }));
        }

        let mut app = AppForm::new();
        app.tokens = form.tokens.clone();
        app.name = form.name.clone();

        if len == 2 {
            match form.params[0].clone() {
                FormParam::Ignore => {
                    app.type_params.push(AppFormTypeParam::Ignore);
                }
                FormParam::TypeKeyword(keyword) => {
                    app.type_params.push(AppFormTypeParam::Keyword(keyword));
                }
                FormParam::TypeSymbol(symbol) => {
                    if is_qualified(&symbol) {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected an unqualified symbol".into(),
                        }));
                    }

                    app.type_params.push(AppFormTypeParam::Symbol(symbol));
                }
                FormParam::Form(form) => {
                    if let Ok(prod) = ProdForm::from_form(&form) {
                        for value in prod.values.iter() {
                            match value {
                                ProdFormValue::TypeKeyword(keyword) => {
                                    app.type_params
                                        .push(AppFormTypeParam::Keyword(keyword.clone()));
                                }
                                ProdFormValue::TypeSymbol(symbol) => {
                                    if is_qualified(&symbol) {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: form.loc(),
                                            desc: "expected an unqualified symbol".into(),
                                        }));
                                    }

                                    app.type_params
                                        .push(AppFormTypeParam::Symbol(symbol.clone()));
                                }
                                ProdFormValue::TypeForm(form) => {
                                    app.type_params.push(AppFormTypeParam::Form(form.clone()));
                                }
                                _ => {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected a product of type symbols or type forms"
                                            .into(),
                                    }));
                                }
                            }
                        }
                    } else {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected a product of type symbols".into(),
                        }));
                    }
                }
                x => {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: format!("unexpected type parameter: {}", x.to_string()),
                    }));
                }
            }

            match form.params[1].clone() {
                FormParam::Empty => {}
                FormParam::Prim(prim) => {
                    app.params.push(AppFormParam::Prim(prim));
                }
                FormParam::TypeKeyword(keyword) => {
                    app.params.push(AppFormParam::TypeKeyword(keyword));
                }
                FormParam::ValueSymbol(symbol) => {
                    app.params.push(AppFormParam::ValueSymbol(symbol));
                }
                FormParam::TypeSymbol(symbol) => {
                    app.params.push(AppFormParam::TypeSymbol(symbol));
                }
                FormParam::Form(form) => {
                    if let Ok(prod) = ProdForm::from_form(&form) {
                        for value in prod.values.iter() {
                            match value.clone() {
                                ProdFormValue::TypeKeyword(keyword) => {
                                    app.params.push(AppFormParam::TypeKeyword(keyword));
                                }
                                ProdFormValue::TypeSymbol(symbol) => {
                                    app.params.push(AppFormParam::TypeSymbol(symbol));
                                }
                                ProdFormValue::ValueSymbol(symbol) => {
                                    app.params.push(AppFormParam::ValueSymbol(symbol));
                                }
                                ProdFormValue::TypeForm(form) => {
                                    app.params.push(AppFormParam::TypeForm(form));
                                }
                                ProdFormValue::ValueForm(form) => {
                                    app.params.push(AppFormParam::ValueForm(form));
                                }
                                ProdFormValue::MixedForm(form) => {
                                    app.params.push(AppFormParam::MixedForm(form));
                                }
                                _ => {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected a product of keywords or symbols".into(),
                                    }));
                                }
                            }
                        }
                    } else {
                        if form.is_type_form() {
                            let form = TypeForm::from_form(&form)?;
                            app.params.push(AppFormParam::TypeForm(form));
                        } else if form.is_value_form() {
                            let form = ValueForm::from_form(&form)?;
                            app.params.push(AppFormParam::ValueForm(form));
                        } else {
                            app.params.push(AppFormParam::MixedForm(form));
                        }
                    }
                }
                x => {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: format!("unexpected parameter: {}", x.to_string()),
                    }));
                }
            }
        } else {
            match form.params[0].clone() {
                FormParam::Empty => {}
                FormParam::Prim(prim) => {
                    app.params.push(AppFormParam::Prim(prim));
                }
                FormParam::TypeKeyword(keyword) => {
                    app.params.push(AppFormParam::TypeKeyword(keyword));
                }
                FormParam::ValueSymbol(symbol) => {
                    app.params.push(AppFormParam::ValueSymbol(symbol));
                }
                FormParam::TypeSymbol(symbol) => {
                    app.params.push(AppFormParam::TypeSymbol(symbol));
                }
                FormParam::Form(form) => {
                    if let Ok(prod) = ProdForm::from_form(&form) {
                        for value in prod.values.iter() {
                            match value.clone() {
                                ProdFormValue::Prim(prim) => {
                                    app.params.push(AppFormParam::Prim(prim));
                                }
                                ProdFormValue::TypeKeyword(keyword) => {
                                    app.params.push(AppFormParam::TypeKeyword(keyword));
                                }
                                ProdFormValue::TypeSymbol(symbol) => {
                                    app.params.push(AppFormParam::TypeSymbol(symbol));
                                }
                                ProdFormValue::ValueSymbol(symbol) => {
                                    app.params.push(AppFormParam::ValueSymbol(symbol));
                                }
                                ProdFormValue::TypeForm(form) => {
                                    app.params.push(AppFormParam::TypeForm(form));
                                }
                                ProdFormValue::ValueForm(form) => {
                                    app.params.push(AppFormParam::ValueForm(form));
                                }
                                ProdFormValue::MixedForm(form) => {
                                    app.params.push(AppFormParam::MixedForm(form));
                                }
                                _ => {
                                    return Err(Error::Semantic(SemanticError {
                                        loc: form.loc(),
                                        desc: "expected a product of keywords or symbols".into(),
                                    }));
                                }
                            }
                        }
                    } else {
                        if form.is_type_form() {
                            let form = TypeForm::from_form(&form)?;
                            app.params.push(AppFormParam::TypeForm(form));
                        } else if form.is_value_form() {
                            let form = ValueForm::from_form(&form)?;
                            app.params.push(AppFormParam::ValueForm(form));
                        } else {
                            app.params.push(AppFormParam::MixedForm(form));
                        }
                    }
                }
                x => {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: format!("unexpected parameter: {}", x.to_string()),
                    }));
                }
            }
        }

        Ok(app)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AppForm> {
        let form = Form::from_tokens(tokens)?;

        AppForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<AppForm> {
        let tokens = Tokens::from_str(s)?;

        AppForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        if self.type_params.is_empty() {
            format!("({} {})", self.name, self.params_to_string(),)
        } else {
            format!(
                "({} {} {})",
                self.name,
                self.type_params_to_string(),
                self.params_to_string(),
            )
        }
    }
}

impl fmt::Display for AppForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn app_form_from_str() {
        use super::AppForm;
        use super::AppFormParam;
        use super::AppFormTypeParam;

        let mut s = "(math.+ (prod 0 1 2 3))";

        let mut res = AppForm::from_str(s);

        //assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "math.+".to_string());
        assert!(form.type_params.is_empty());
        assert_eq!(form.params_to_string(), "(prod 0 1 2 3)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap _ ())";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "unwrap".to_string());
        assert_eq!(form.type_params, vec![AppFormTypeParam::Ignore]);
        assert!(form.params.is_empty());
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.params_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap _ stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params, vec![AppFormParam::ValueSymbol("stdIO".into())]);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.params_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap _ \"io error\")";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.params, vec![AppFormParam::Prim("\"io error\"".into())]);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.params_to_string(), "\"io error\"".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap (prod IO Error) stdIO)";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.type_params_to_string(), "(prod IO Error)".to_string());
        assert_eq!(form.params_to_string(), "stdIO".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(unwrap (prod (Fun (Prod Int Int) Int) Error) (fun (prod a b) (math.+ a b)))";

        res = AppForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.type_params_to_string(),
            "(prod (Fun (Prod Int Int) Int) Error)".to_string()
        );
        assert_eq!(
            form.params_to_string(),
            "(fun (prod a b) (math.+ a b))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
