use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::forms::pair_form::{PairForm, PairFormValue};
use crate::value::SimpleValue;
use crate::value::Type;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum MapFormEntry {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    PairForm(Box<PairForm>),
}

impl Default for MapFormEntry {
    fn default() -> MapFormEntry {
        MapFormEntry::Empty(SimpleValue::new())
    }
}

impl MapFormEntry {
    pub fn file(&self) -> String {
        match self {
            MapFormEntry::Ignore(ignore) => ignore.file(),
            MapFormEntry::Empty(empty) => empty.file(),
            MapFormEntry::PairForm(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            MapFormEntry::Ignore(ignore) => ignore.loc(),
            MapFormEntry::Empty(empty) => empty.loc(),
            MapFormEntry::PairForm(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            MapFormEntry::Ignore(_) => "_".into(),
            MapFormEntry::Empty(_) => "()".into(),
            MapFormEntry::PairForm(form) => form.to_string(),
        }
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        match self {
            MapFormEntry::Ignore(_) | MapFormEntry::Empty(_) => {}
            MapFormEntry::PairForm(form) => value_vars.extend(form.all_value_variables()),
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        match self {
            MapFormEntry::Ignore(_) | MapFormEntry::Empty(_) => {}
            MapFormEntry::PairForm(form) => type_vars.extend(form.all_type_variables()),
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        match self {
            MapFormEntry::Ignore(_) | MapFormEntry::Empty(_) => {}
            MapFormEntry::PairForm(form) => vars.extend(form.all_variables()),
        }

        vars
    }
}

impl fmt::Display for MapFormEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct MapForm {
    pub tokens: Box<Tokens>,
    pub entries: Vec<MapFormEntry>,
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
        if self.is_empty() {
            0
        } else {
            self.entries.len()
        }
    }

    pub fn is_ignore_literal(&self) -> bool {
        if self.entries.len() != 1 {
            return false;
        }

        matches!(self.entries[0], MapFormEntry::Ignore(_))
    }

    pub fn is_empty_literal(&self) -> bool {
        if self.entries.len() != 1 {
            return false;
        }

        matches!(self.entries[0], MapFormEntry::Empty(_))
    }

    pub fn is_proper_map(&self) -> bool {
        !matches!(
            self.entries[0],
            MapFormEntry::Empty(_) | MapFormEntry::Ignore(_)
        )
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() || self.is_empty_literal()
    }

    pub fn as_map(&self) -> Result<BTreeMap<PairFormValue, PairFormValue>> {
        let mut map = BTreeMap::new();

        if self.len() == 1 {
            match self.entries[0].clone() {
                MapFormEntry::Empty(_) | MapFormEntry::Ignore(_) => {}
                MapFormEntry::PairForm(form) => {
                    map.insert(form.first.clone(), form.second.clone());
                }
            }
        } else {
            for entry in self.entries.iter() {
                match entry {
                    MapFormEntry::Empty(_) | MapFormEntry::Ignore(_) => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: entry.loc(),
                            desc: "unexpected map entry".into(),
                        }));
                    }
                    MapFormEntry::PairForm(form) => {
                        map.insert(form.first.clone(), form.second.clone());
                    }
                }
            }
        }

        Ok(map)
    }

    pub fn get_from_pair_form_value_key(
        &self,
        key: &PairFormValue,
    ) -> Result<Option<PairFormValue>> {
        Ok(self.as_map()?.get(key).map(|v| v.to_owned()))
    }

    pub fn get(&self, key: &str) -> Result<Option<PairFormValue>> {
        for entry in self.as_map()?.iter() {
            if entry.0.to_string() == key {
                return Ok(Some(entry.1.to_owned()));
            }
        }

        Ok(None)
    }

    pub fn contains_from_pair_form_value_key(&self, key: &PairFormValue) -> Result<bool> {
        Ok(self.as_map()?.contains_key(key))
    }

    pub fn contains(&self, key: &str) -> Result<bool> {
        for entry in self.as_map()?.iter() {
            if entry.0.to_string() == key {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn can_be_parameter(&self) -> bool {
        for entry in self.entries.iter() {
            match entry {
                MapFormEntry::Ignore(_) | MapFormEntry::Empty(_) => {
                    return false;
                }
                MapFormEntry::PairForm(form) => {
                    if !form.can_be_parameter() {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn entries_to_string(&self) -> String {
        self.entries
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        let mut params = vec![];

        for entry in self.entries.iter() {
            if let MapFormEntry::PairForm(form) = entry.clone() {
                params.extend(form.all_parameters());
            }
        }

        params
    }

    pub fn all_value_variables(&self) -> Vec<SimpleValue> {
        let mut value_vars = vec![];

        for entry in self.entries.iter() {
            value_vars.extend(entry.all_value_variables());
        }

        value_vars
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        let mut type_vars = vec![];

        for entry in self.entries.iter() {
            type_vars.extend(entry.all_type_variables());
        }

        type_vars
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        let mut vars = vec![];

        for entry in self.entries.iter() {
            vars.extend(entry.all_variables());
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
                desc: "expected at least one entry".into(),
            }));
        }

        let mut map = MapForm::new();
        map.tokens = form.tokens.clone();

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value {
                    SimpleValue::Ignore(_) => {
                        map.entries.push(MapFormEntry::Ignore(value));
                    }
                    SimpleValue::Empty(_) => {
                        if form.tail.len() > 1 {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected at most one value if the first is an empty literal"
                                    .into(),
                            }));
                        }

                        map.entries.push(MapFormEntry::Empty(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: x.loc(),
                            desc: "unxexpected value".into(),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if let Ok(form) = PairForm::from_form(&form) {
                        map.entries.push(MapFormEntry::PairForm(Box::new(form)));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected a pair form".into(),
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
        format!("(map {})", self.entries_to_string())
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

        assert!(form.is_empty());
        assert!(form.is_empty_literal());
        assert_eq!(form.len(), 0);
        assert_eq!(form.entries_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(map _)";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert!(!form.is_empty());
        assert!(form.is_ignore_literal());
        assert_eq!(form.len(), 1);
        assert_eq!(form.entries_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(map (pair a A))";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.len(), 1);
        assert!(form.is_proper_map());
        assert_eq!(form.entries_to_string(), "(pair a A)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.contains("a").is_ok());
        assert!(form.contains("a").unwrap());
        assert_eq!(form.get("a").unwrap().unwrap().to_string(), "A".to_string());

        s = "(map (pair moduleX.X y))";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.len(), 1);
        assert!(form.is_proper_map());
        assert_eq!(form.entries_to_string(), "(pair moduleX.X y)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.contains("moduleX.X").is_ok());
        assert!(form.contains("moduleX.X").unwrap());
        assert_eq!(
            form.get("moduleX.X").unwrap().unwrap().to_string(),
            "y".to_string()
        );

        s = "(map (pair moduleX.X y) (pair math.+ default))";

        res = MapForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.len(), 2);
        assert!(form.is_proper_map());
        assert_eq!(
            form.entries
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec![
                "(pair moduleX.X y)".to_string(),
                "(pair math.+ default)".to_string()
            ]
        );
        assert_eq!(
            form.entries_to_string(),
            "(pair moduleX.X y) (pair math.+ default)".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.contains("moduleX.X").is_ok());
        assert!(form.contains("moduleX.X").unwrap());
        assert_eq!(
            form.get("moduleX.X").unwrap().unwrap().to_string(),
            "y".to_string()
        );
        assert!(form.contains("math.+").is_ok());
        assert!(form.contains("math.+").unwrap());
        assert_eq!(
            form.get("math.+").unwrap().unwrap().to_string(),
            "default".to_string()
        );
    }
}
