use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::forms::list_form::{ListForm, ListFormValue};
use crate::value::SimpleValue;
use crate::value::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum ExportFormDef {
    Empty(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
}

impl Default for ExportFormDef {
    fn default() -> ExportFormDef {
        ExportFormDef::Empty(SimpleValue::new())
    }
}

impl ExportFormDef {
    pub fn file(&self) -> String {
        match self {
            ExportFormDef::Empty(empty) => empty.file(),
            ExportFormDef::ValueSymbol(symbol) => symbol.file(),
            ExportFormDef::TypeSymbol(symbol) => symbol.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ExportFormDef::Empty(empty) => empty.loc(),
            ExportFormDef::ValueSymbol(symbol) => symbol.loc(),
            ExportFormDef::TypeSymbol(symbol) => symbol.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ExportFormDef::Empty(_) => "()".into(),
            ExportFormDef::ValueSymbol(symbol) => symbol.to_string(),
            ExportFormDef::TypeSymbol(symbol) => symbol.to_string(),
        }
    }
}

impl fmt::Display for ExportFormDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ExportForm {
    pub tokens: Box<Tokens>,
    pub defs: Vec<ExportFormDef>,
}

impl ExportForm {
    pub fn new() -> ExportForm {
        ExportForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
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
            _ => "()".to_string(),
        }
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        for def in self.defs.iter() {
            if let ExportFormDef::ValueSymbol(value) = def.clone() {
                value_vars.push(value);
            }
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        for def in self.defs.iter() {
            if let ExportFormDef::TypeSymbol(value) = def.clone() {
                type_vars.push(Type::from_simple_value(&value).unwrap());
            }
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for def in self.defs.iter() {
            match def.clone() {
                ExportFormDef::ValueSymbol(value) => {
                    vars.push(value);
                }
                ExportFormDef::TypeSymbol(value) => {
                    vars.push(value);
                }
                _ => {}
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<ExportForm> {
        if form.head.to_string() != "export" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an export keyword".into(),
            }));
        }

        if form.tail.len() != 1 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected one or more exported symbols".into(),
            }));
        }

        let mut export = ExportForm::new();
        export.tokens = form.tokens.clone();

        for param in form.tail.clone() {
            match param {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Empty(_) => {
                        export.defs.push(ExportFormDef::Empty(value));
                    }
                    SimpleValue::TypeSymbol(_) => {
                        export.defs.push(ExportFormDef::TypeSymbol(value));
                    }
                    SimpleValue::ValueSymbol(_) => {
                        export.defs.push(ExportFormDef::ValueSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unexpected an unqualified symbol or an empty literal".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    let list = ListForm::from_form(&form)?;

                    for value in list.values {
                        match value {
                            ListFormValue::ValueSymbol(symbol) => {
                                export.defs.push(ExportFormDef::ValueSymbol(symbol));
                            }
                            ListFormValue::TypeSymbol(symbol) => {
                                export.defs.push(ExportFormDef::TypeSymbol(symbol));
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
        }

        Ok(export)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ExportForm> {
        let form = Form::from_tokens(tokens)?;

        ExportForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ExportForm> {
        let tokens = Tokens::from_str(s)?;

        ExportForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(export {})", self.defs_to_string(),)
    }
}

impl fmt::Display for ExportForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for ExportForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn export_form_from_str() {
        use super::ExportForm;

        let mut s = "(export A)";

        let mut res = ExportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.defs_to_string(), "A".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(export (list b C d E))";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.defs_to_string(), "(list b C d E)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(export ())";

        res = ExportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.defs_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
