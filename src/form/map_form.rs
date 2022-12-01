use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::prod_form::ProdForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MapFormValue {
    Empty(SimpleValue),
    ProdForm(Box<ProdForm>),
}

impl Default for MapFormValue {
    fn default() -> MapFormValue {
        MapFormValue::Empty(SimpleValue::new())
    }
}

impl MapFormValue {
    pub fn file(&self) -> String {
        match self {
            MapFormValue::Empty(empty) => empty.file(),
            MapFormValue::ProdForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            MapFormValue::Empty(empty) => empty.loc(),
            MapFormValue::ProdForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            MapFormValue::Empty(_) => "()".into(),
            MapFormValue::ProdForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for MapFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct MapForm {
    pub tokens: Box<Tokens>,
    pub values: Vec<MapFormValue>,
}

impl MapForm {
    pub fn new() -> MapForm {
        MapForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn values_to_string(&self) -> String {
        self.values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for value in self.values.iter() {
            if let MapFormValue::ProdForm(form) = value.clone() {
                params.extend(form.all_parameters());
            }
        }

        params
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for value in self.values.iter() {
            if let MapFormValue::ProdForm(form) = value.clone() {
                vars.extend(form.all_variables());
            }
        }

        vars
    }

    pub fn from_form(form: &Form) -> Result<MapForm> {
        if form.head.to_string() != "map" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a map keyword".into(),
            }));
        }

        if form.tail.is_empty() {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least one value".into(),
            }));
        }

        let mut map = MapForm::new();
        map.tokens = form.tokens.clone();

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Empty(_) => {
                        if form.tail.len() > 1 {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected at most one value if the first is an empty literal"
                                    .into(),
                            }));
                        }

                        map.values.push(MapFormValue::Empty(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unxexpected value".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = ProdForm::from_form(&form) {
                        map.values.push(MapFormValue::ProdForm(Box::new(form)));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected a product form".into(),
                        }));
                    }
                }
            }
        }

        Ok(map)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<MapForm> {
        let form = Form::from_tokens(tokens)?;

        MapForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<MapForm> {
        let tokens = Tokens::from_str(s)?;

        MapForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(map {})", self.values_to_string())
    }
}

impl fmt::Display for MapForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for MapForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn map_form_from_str() {
        use super::MapForm;

        let mut s = "(map ())";

        let mut res = MapForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["()".to_string()]
        );
        assert_eq!(form.values_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(map (prod a A))";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["(prod a A)".to_string()]
        );
        assert_eq!(form.values_to_string(), "(prod a A)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(map (prod moduleX.X y))";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["(prod moduleX.X y)".to_string()]
        );
        assert_eq!(form.values_to_string(), "(prod moduleX.X y)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(map (prod moduleX.X y) (prod math.+ default))";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec![
                "(prod moduleX.X y)".to_string(),
                "(prod math.+ default)".to_string()
            ]
        );
        assert_eq!(
            form.values_to_string(),
            "(prod moduleX.X y) (prod math.+ default)".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
