use crate::result::Result;
use crate::syntax::Keyword;
use crate::typing::Type;
use crate::value::Value;
use crate::values::Values;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct STElement {
    pub name: String,
    pub value: Value,
    pub file: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolTable {
    pub files: BTreeSet<String>,
    pub incl_paths: BTreeSet<String>,
    pub def_types: BTreeSet<String>,
    pub def_values: BTreeSet<String>,

    pub includes: BTreeMap<String, Vec<STElement>>,
    pub types: BTreeMap<Type, Vec<STElement>>,
    pub sigs: BTreeMap<String, Vec<STElement>>,
    pub sums: BTreeMap<String, Vec<STElement>>,
    pub prods: BTreeMap<String, Vec<STElement>>,
    pub funs: BTreeMap<String, Vec<STElement>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn from_values(values: &Values) -> Result<Self> {
        let mut files = BTreeSet::<String>::new();
        let mut def_values = BTreeSet::<String>::new();
        let mut def_types = BTreeSet::<String>::new();
        let mut incl_paths = BTreeSet::<String>::new();

        for value in values.clone().into_iter() {
            if let Some(file) = value.token.file() {
                files.insert(file);
            }

            if let Some(typing) = value.typing {
                if typing == Type::Builtin {
                    let arg = value.children[1].name.clone().unwrap();
                    let keyword = Keyword::from_str(&value.name.unwrap())?;

                    match keyword {
                        Keyword::Include => {
                            incl_paths.insert(arg);
                        }
                        Keyword::Defvar | Keyword::Defsum | Keyword::Defprod | Keyword::Defun => {
                            def_values.insert(arg);
                        }
                        Keyword::Deftype | Keyword::Defsig => {
                            def_types.insert(arg);
                        }
                    }
                }
            }
        }

        let mut st = SymbolTable::new();

        st.files = files;
        st.incl_paths = incl_paths;
        st.def_types = def_types;
        st.def_values = def_values;

        Ok(st)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn from_values() {
        use super::SymbolTable;
        use crate::values::Values;

        let s = "(include std.io)";

        let values = Values::from_str(s).unwrap();

        let res = SymbolTable::from_values(&values);

        assert!(res.is_ok());
    }
}
