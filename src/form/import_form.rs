use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormParam};
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::type_form::TypeForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::symbol_name;
use crate::syntax::{is_keyword, is_path_symbol, is_qualified};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ImportFormTypeParam {
    Ignore,
    Empty,
    Keyword(String),
    Symbol(String),
    Form(Box<TypeForm>),
}

impl Default for ImportFormTypeParam {
    fn default() -> ImportFormTypeParam {
        ImportFormTypeParam::Ignore
    }
}

impl ImportFormTypeParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ImportFormTypeParam::Ignore => "_".into(),
            ImportFormTypeParam::Empty => "()".into(),
            ImportFormTypeParam::Keyword(keyword) => keyword.clone(),
            ImportFormTypeParam::Symbol(symbol) => symbol.clone(),
            ImportFormTypeParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ImportFormTypeParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ImportFormDef {
    Ignore,
    Empty,
    ValueSymbol(String),
    TypeSymbol(String),
}

impl Default for ImportFormDef {
    fn default() -> ImportFormDef {
        ImportFormDef::Ignore
    }
}

impl ImportFormDef {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ImportFormDef::Ignore => "_".into(),
            ImportFormDef::Empty => "()".into(),
            ImportFormDef::ValueSymbol(symbol) => symbol.clone(),
            ImportFormDef::TypeSymbol(symbol) => symbol.clone(),
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
    pub module: String,
    pub qualifier: Option<String>,
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
        match form.params[idx].clone() {
            FormParam::ValueSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                if is_keyword(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected keyword".into(),
                    }));
                }

                self.qualifier = Some(symbol);
            }
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
        match form.params[idx].clone() {
            FormParam::Ignore => {
                self.type_params.push(ImportFormTypeParam::Ignore);
            }
            FormParam::Empty => {
                self.type_params.push(ImportFormTypeParam::Empty);
            }
            FormParam::TypeKeyword(keyword) => {
                self.type_params.push(ImportFormTypeParam::Keyword(keyword));
            }
            FormParam::TypeSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                self.type_params.push(ImportFormTypeParam::Symbol(symbol));
            }
            FormParam::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::TypeKeyword(keyword) => {
                                self.type_params.push(ImportFormTypeParam::Keyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                self.type_params.push(ImportFormTypeParam::Symbol(symbol));
                            }
                            ProdFormValue::TypeForm(form) => {
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
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: format!("unexpected type parameter: {}", x.to_string()),
                }));
            }
        }

        Ok(())
    }

    fn parse_defs(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.params[idx].clone() {
            FormParam::Ignore => {
                self.defs.push(ImportFormDef::Ignore);
            }
            FormParam::Empty => {
                self.defs.push(ImportFormDef::Empty);
            }
            FormParam::ValueSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                if is_keyword(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected keyword".into(),
                    }));
                }

                self.defs.push(ImportFormDef::ValueSymbol(symbol));
            }
            FormParam::TypeSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                if is_keyword(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected keyword".into(),
                    }));
                }

                self.defs.push(ImportFormDef::TypeSymbol(symbol));
            }
            FormParam::Form(form) => {
                let prod = ProdForm::from_form(&form)?;

                for value in prod.values {
                    match value {
                        ProdFormValue::ValueSymbol(symbol) => {
                            if is_qualified(&symbol) {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected an unqualified symbol".into(),
                                }));
                            }

                            if is_keyword(&symbol) {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "unexpected keyword".into(),
                                }));
                            }

                            self.defs.push(ImportFormDef::ValueSymbol(symbol));
                        }
                        ProdFormValue::TypeSymbol(symbol) => {
                            if is_qualified(&symbol) {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "expected an unqualified symbol".into(),
                                }));
                            }

                            if is_keyword(&symbol) {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: form.loc(),
                                    desc: "unexpected keyword".into(),
                                }));
                            }

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
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a product of symbols or a value symbol".into(),
                }));
            }
        }

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<ImportForm> {
        if form.name != "import" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected an import keyword".into(),
            }));
        }

        let mut import = ImportForm::new();
        import.tokens = form.tokens.clone();

        let len = form.params.len();

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

        let module = form.params[0].to_string();

        if is_path_symbol(&module) {
            if is_keyword(&symbol_name(&module)) {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "a path cannot end with a keyword".into(),
                }));
            }
        } else if is_keyword(&module) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a module path".into(),
            }));
        }

        import.module = module;

        if len > 1 {
            match len {
                2 => {
                    import.parse_defs(&form, 1)?;
                }
                3 => {
                    import.parse_defs(&form, 1)?;
                    import.parse_qualifier(&form, 2)?;
                }
                4 => {
                    import.parse_defs(&form, 1)?;
                    import.parse_type_params(&form, 2)?;
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

    pub fn from_str(s: &str) -> Result<ImportForm> {
        let tokens = Tokens::from_str(s)?;

        ImportForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        if let Some(ref qualifier) = self.qualifier {
            if self.type_params.is_empty() {
                format!(
                    "(import {} {} {})",
                    self.module,
                    self.defs_to_string(),
                    qualifier
                )
            } else {
                format!(
                    "(import {} {} {} {})",
                    self.module,
                    self.defs_to_string(),
                    self.type_params_to_string(),
                    qualifier
                )
            }
        } else if self.defs.is_empty() {
            format!("(import {})", self.module)
        } else {
            format!("(import {} {})", self.module, self.defs_to_string())
        }
    }
}

impl fmt::Display for ImportForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn import_form_from_str() {
        use super::ImportForm;
        use super::ImportFormDef;
        use super::ImportFormTypeParam;

        let mut s = "(import std.x (prod a B c D) x)";

        let mut res = ImportForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert!(form.type_params.is_empty());
        assert_eq!(form.type_params_to_string(), "".to_string());
        assert_eq!(form.defs_to_string(), "(prod a B c D)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x () _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params, vec![ImportFormTypeParam::Ignore]);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs, vec![ImportFormDef::Empty]);
        assert_eq!(form.defs_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ () x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params, vec![ImportFormTypeParam::Empty]);
        assert_eq!(form.type_params_to_string(), "()".to_string());
        assert_eq!(form.defs, vec![ImportFormDef::Ignore]);
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.defs, vec![ImportFormDef::Ignore]);
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert!(form.type_params.is_empty());
        assert_eq!(form.type_params_to_string(), "".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x () _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params, vec![ImportFormTypeParam::Ignore]);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert!(form.type_params.is_empty());
        assert!(form.defs.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x X)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, None);
        assert_eq!(form.defs_to_string(), "X".to_string());
        assert!(form.type_params.is_empty());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x x x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert!(form.type_params.is_empty());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x x _ x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params, vec![ImportFormTypeParam::Ignore]);
        assert_eq!(form.type_params_to_string(), "_".to_string());
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x x (prod T Q) x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params_to_string(), "(prod T Q)");
        assert_eq!(form.defs_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x (prod A b C) (prod T Q) x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params_to_string(), "(prod T Q)");
        assert_eq!(form.defs_to_string(), "(prod A b C)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(import std.x _ (prod Char Float) x)";

        res = ImportForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.module, "std.x".to_string());
        assert_eq!(form.qualifier, Some("x".into()));
        assert_eq!(form.type_params_to_string(), "(prod Char Float)");
        assert_eq!(form.defs_to_string(), "_".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
