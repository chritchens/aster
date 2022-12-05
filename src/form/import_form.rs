use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::list_form::{ListForm, ListFormValue};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::types::Type;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
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
    pub fn file(&self) -> String {
        match self {
            ImportFormDef::Ignore(ignore) => ignore.file(),
            ImportFormDef::Empty(empty) => empty.file(),
            ImportFormDef::ValueSymbol(symbol) => symbol.file(),
            ImportFormDef::TypeSymbol(symbol) => symbol.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ImportFormDef::Ignore(ignore) => ignore.loc(),
            ImportFormDef::Empty(empty) => empty.loc(),
            ImportFormDef::ValueSymbol(symbol) => symbol.loc(),
            ImportFormDef::TypeSymbol(symbol) => symbol.loc(),
        }
    }

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ImportForm {
    pub tokens: Box<Tokens>,
    pub module: SimpleValue,
    pub qualifier: Option<SimpleValue>,
    pub type_variables: Vec<Type>,
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

    pub fn type_variables_to_string(&self) -> String {
        match self.type_variables.len() {
            0 => "()".into(),
            1 => self.type_variables[0].to_string(),
            _ => format!(
                "(list {})",
                self.type_variables
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }

    pub fn defs_to_string(&self) -> String {
        match self.defs.len() {
            1 => self.defs[0].to_string(),
            x if x > 1 => format!(
                "(list {})",
                self.defs
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "".to_string(),
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for def in self.defs.iter() {
            match def.clone() {
                ImportFormDef::Ignore(value) => {
                    params.push(value);
                }
                ImportFormDef::Empty(value) => {
                    params.push(value);
                }
                ImportFormDef::ValueSymbol(value) => {
                    params.push(value);
                }
                ImportFormDef::TypeSymbol(value) => {
                    params.push(value);
                }
            }
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        vec![]
    }

    fn parse_qualifier(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    self.qualifier = Some(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected an unqualified value symbol".into(),
                    }));
                }
            },
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected an unqualified value symbol".into(),
                }));
            }
        }

        Ok(())
    }

    fn parse_type_variables(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => {
                if value.to_string() != "()" {
                    let simple_type = Type::from_simple_value(&value)?;
                    self.type_variables.push(simple_type);
                }
            }
            FormTailElement::Form(form) => {
                let list_form = ListForm::from_form(&form)?;

                for value in list_form.values.iter() {
                    match value {
                        ListFormValue::Ignore(value) => {
                            let simple_type = Type::from_simple_value(&value)?;
                            self.type_variables.push(simple_type);
                        }
                        ListFormValue::TypeKeyword(value) => {
                            let simple_type = Type::from_simple_value(&value)?;
                            self.type_variables.push(simple_type);
                        }
                        ListFormValue::TypeSymbol(value) => {
                            let simple_type = Type::from_simple_value(&value)?;
                            self.type_variables.push(simple_type);
                        }
                        ListFormValue::TypePathSymbol(value) => {
                            let simple_type = Type::from_simple_value(&value)?;
                            self.type_variables.push(simple_type);
                        }
                        ListFormValue::TypesForm(form) => {
                            let form_type = Type::from_form(&form.as_form())?;
                            self.type_variables.push(form_type);
                        }
                        _ => {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: value.loc(),
                                desc: "unexpected value".into(),
                            }));
                        }
                    }
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
                        loc: x.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                let list = ListForm::from_form(&form)?;

                for value in list.values {
                    match value {
                        ListFormValue::ValueSymbol(symbol) => {
                            self.defs.push(ImportFormDef::ValueSymbol(symbol));
                        }
                        ListFormValue::TypeSymbol(symbol) => {
                            self.defs.push(ImportFormDef::TypeSymbol(symbol));
                        }
                        x => {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: x.loc(),
                                desc: "expected an unqualified symbol".into(),
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
                loc: form.head.loc(),
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
                desc:
                    "expected at most a module, type variables, imported symbols, and a qualifier"
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
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected a value symbol".into(),
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

        if len > 1 {
            match len {
                2 => {
                    import.parse_type_variables(&form, 1)?;
                }
                3 => {
                    import.parse_type_variables(&form, 1)?;
                    import.parse_defs(&form, 2)?;
                }
                4 => {
                    import.parse_type_variables(&form, 1)?;
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
                self.type_variables_to_string(),
                self.defs_to_string(),
                qualifier
            )
        } else if self.defs.is_empty() {
            if self.type_variables.is_empty() {
                format!("(import {})", self.module)
            } else {
                format!(
                    "(import {} {})",
                    self.module,
                    self.type_variables_to_string()
                )
            }
        } else {
            format!(
                "(import {} {} {})",
                self.module,
                self.type_variables_to_string(),
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

        let mut s = "(import std.x _ (list a B c D) x)";

        let mut res = ImportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_variables_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "(list a B c D)".to_string());
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
        assert_eq!(form.type_variables_to_string(), "_".to_string());
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
        assert_eq!(form.type_variables_to_string(), "()".to_string());
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_variables_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ ())";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_variables_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert!(form.type_variables.is_empty());
        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x X)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.type_variables_to_string(), "X".to_string());
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
        assert_eq!(form.type_variables_to_string(), "_".to_string());
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
        assert_eq!(form.type_variables_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (list T Q) x x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_variables_to_string(), "(list T Q)");
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (list T Q) (list A b C) x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_variables_to_string(), "(list T Q)");
        assert_eq!(form.defs_to_string(), "(list A b C)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (list Char Float) _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module.to_string(), "std.x".to_string());
        assert_eq!(
            form.qualifier.as_ref().map(|q| q.to_string()),
            Some("x".into())
        );
        assert_eq!(form.type_variables_to_string(), "(list Char Float)");
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
