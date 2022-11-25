use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::module_form::ModuleFormTypeParam;
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::simple_value::SimpleValue;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

pub type ImportFormTypeParam = ModuleFormTypeParam;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ImportFormDef {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
}

impl Default for ImportFormDef {
    fn default() -> ImportFormDef {
        ImportFormDef::Empty(SimpleValue::new())
    }
}

impl ImportFormDef {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ImportFormDef::Ignore(_) => "_".into(),
            ImportFormDef::Empty(_) => "()".into(),
            ImportFormDef::ValueSymbol(symbol) => symbol.to_string(),
            ImportFormDef::TypeSymbol(symbol) => symbol.to_string(),
        }
    }
}

impl fmt::Display for ImportFormDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ImportForm {
    pub tokens: Box<Tokens>,
    pub module: SimpleValue,
    pub qualifier: Option<SimpleValue>,
    pub type_params: Vec<ImportFormTypeParam>,
    pub defs: Vec<ImportFormDef>,
}

impl ImportForm {
    pub fn new() -> ImportForm {
        ImportForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn type_params_to_string(&self) -> String {
        match self.type_params.len() {
            1 => self.type_params[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.type_params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "".to_string(),
        }
    }

    pub fn defs_to_string(&self) -> String {
        match self.defs.len() {
            1 => self.defs[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.defs
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "".to_string(),
        }
    }

    fn parse_qualifier(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    self.qualifier = Some(value);
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified value symbol".into(),
                    }));
                }
            },
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a value symbol".into(),
                }));
            }
        }

        Ok(())
    }

    fn parse_type_params(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Ignore(_) => {
                    self.type_params.push(ImportFormTypeParam::Ignore(value));
                }
                SimpleValue::Empty(_) => {
                    self.type_params.push(ImportFormTypeParam::Empty(value));
                }
                SimpleValue::TypeKeyword(_) => {
                    self.type_params.push(ImportFormTypeParam::Keyword(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    self.type_params.push(ImportFormTypeParam::Symbol(value));
                }
                SimpleValue::TypePathSymbol(_) => {
                    self.type_params
                        .push(ImportFormTypeParam::PathSymbol(value));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected type parameter: {}", x.to_string()),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::TypeKeyword(keyword) => {
                                self.type_params.push(ImportFormTypeParam::Keyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                self.type_params.push(ImportFormTypeParam::Symbol(symbol));
                            }
                            ProdFormValue::TypePathSymbol(symbol) => {
                                self.type_params
                                    .push(ImportFormTypeParam::PathSymbol(symbol));
                            }
                            ProdFormValue::TypesForm(form) => {
                                self.type_params.push(ImportFormTypeParam::Form(form));
                            }
                            _ => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected a product of type symbols or type forms".into(),
                                }));
                            }
                        }
                    }
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a product of type symbols".into(),
                    }));
                }
            }
        }

        Ok(())
    }

    fn parse_defs(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Ignore(_) => {
                    self.defs.push(ImportFormDef::Ignore(value));
                }
                SimpleValue::Empty(_) => {
                    self.defs.push(ImportFormDef::Empty(value));
                }
                SimpleValue::ValueSymbol(_) => {
                    self.defs.push(ImportFormDef::ValueSymbol(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    self.defs.push(ImportFormDef::TypeSymbol(value));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected value: {}", x.to_string()),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                let prod = ProdForm::from_form(&form)?;

                for value in prod.values {
                    match value {
                        ProdFormValue::ValueSymbol(symbol) => {
                            self.defs.push(ImportFormDef::ValueSymbol(symbol));
                        }
                        ProdFormValue::TypeSymbol(symbol) => {
                            self.defs.push(ImportFormDef::TypeSymbol(symbol));
                        }
                        _ => {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a product of value or type symbols".into(),
                            }));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<ImportForm> {
        if form.head.to_string() != "import" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected an import keyword".into(),
            }));
        }

        let mut import = ImportForm::new();
        import.tokens = form.tokens.clone();

        let len = form.tail.len();

        if len == 0 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least a module name".into(),
            }));
        }

        if len > 4 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at most a module, an optional product of types, a product of value symbols or a value symbol, and a value symbol"
                    .into(),
            }));
        }

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    import.module = value;
                }
                SimpleValue::ValuePathSymbol(_) => {
                    import.module = value;
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a value symbol or a value path symbol".into(),
                    }));
                }
            },
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a value symbol or a value path symbol".into(),
                }));
            }
        }

        if len > 1 {
            match len {
                2 => {
                    import.parse_type_params(&form, 1)?;
                }
                3 => {
                    import.parse_type_params(&form, 1)?;
                    import.parse_defs(&form, 2)?;
                }
                4 => {
                    import.parse_type_params(&form, 1)?;
                    import.parse_defs(&form, 2)?;
                    import.parse_qualifier(&form, 3)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(import)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ImportForm> {
        let form = Form::from_tokens(tokens)?;

        ImportForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ImportForm> {
        let tokens = Tokens::from_str(s)?;

        ImportForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        if let Some(ref qualifier) = self.qualifier {
            format!(
                "(import {} {} {} {})",
                self.module,
                self.type_params_to_string(),
                self.defs_to_string(),
                qualifier
            )
        } else if self.defs.is_empty() {
            if self.type_params.is_empty() {
                format!("(import {})", self.module)
            } else {
                format!("(import {} {})", self.module, self.type_params_to_string())
            }
        } else {
            format!(
                "(import {} {} {})",
                self.module,
                self.type_params_to_string(),
                self.defs_to_string()
            )
        }
    }
}

impl fmt::Display for ImportForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for ImportForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn import_form_from_str() {
        use super::ImportForm;

        let mut s = "(import std.x _ (prod a B c D) x)";

        let mut res = ImportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "(prod a B c D)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ () x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x () _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "()".to_string());
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ ())";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert!(form.type_params.is_empty());
        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x X)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_params_to_string(), "X".to_string());
        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ x x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ x x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (prod T Q) x x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "(prod T Q)");
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (prod T Q) (prod A b C) x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "(prod T Q)");
        assert_eq!(form.defs_to_string(), "(prod A b C)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (prod Char Float) _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_params_to_string(), "(prod Char Float)");
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
