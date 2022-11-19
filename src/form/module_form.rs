use crate::error::{Error, SyntacticError};
use crate::form::attrs_form::AttrsForm;
use crate::form::export_form::ExportForm;
use crate::form::form::{Form, FormParam};
use crate::form::import_form::ImportForm;
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::sig_form::SigForm;
use crate::form::type_form::TypeForm;
use crate::form::types_form::TypesForm;
use crate::form::val_form::ValForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_qualified, is_value_symbol};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModuleFormTypeParam {
    Ignore,
    Empty,
    Keyword(String),
    Symbol(String),
    Form(Box<TypesForm>),
}

impl Default for ModuleFormTypeParam {
    fn default() -> ModuleFormTypeParam {
        ModuleFormTypeParam::Ignore
    }
}

impl ModuleFormTypeParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ModuleFormTypeParam::Ignore => "_".into(),
            ModuleFormTypeParam::Empty => "()".into(),
            ModuleFormTypeParam::Keyword(keyword) => keyword.clone(),
            ModuleFormTypeParam::Symbol(symbol) => symbol.clone(),
            ModuleFormTypeParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ModuleFormTypeParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModuleFormEntry {
    Empty,
    ImportForm(Box<ImportForm>),
    ExportForm(Box<ExportForm>),
    AttrsForm(Box<AttrsForm>),
    TypeForm(Box<TypeForm>),
    SigForm(Box<SigForm>),
    ValForm(Box<ValForm>),
}

impl Default for ModuleFormEntry {
    fn default() -> ModuleFormEntry {
        ModuleFormEntry::Empty
    }
}

impl ModuleFormEntry {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ModuleFormEntry::Empty => "()".into(),
            ModuleFormEntry::ImportForm(form) => form.to_string(),
            ModuleFormEntry::ExportForm(form) => form.to_string(),
            ModuleFormEntry::AttrsForm(form) => form.to_string(),
            ModuleFormEntry::TypeForm(form) => form.to_string(),
            ModuleFormEntry::SigForm(form) => form.to_string(),
            ModuleFormEntry::ValForm(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ModuleFormEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ModuleForm {
    pub tokens: Box<Tokens>,
    pub name: String,
    pub type_params: Vec<ModuleFormTypeParam>,
    pub entries: Vec<ModuleFormEntry>,
}

impl ModuleForm {
    pub fn new() -> ModuleForm {
        ModuleForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn entry_as_import(&self, idx: usize) -> Option<Box<ImportForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            ModuleFormEntry::ImportForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_export(&self, idx: usize) -> Option<Box<ExportForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            ModuleFormEntry::ExportForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_type(&self, idx: usize) -> Option<Box<TypeForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            ModuleFormEntry::TypeForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_attributes(&self, idx: usize) -> Option<Box<AttrsForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            ModuleFormEntry::AttrsForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_signature(&self, idx: usize) -> Option<Box<SigForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            ModuleFormEntry::SigForm(form) => Some(form),
            _ => None,
        }
    }

    pub fn entry_as_value(&self, idx: usize) -> Option<Box<ValForm>> {
        if idx > self.entries.len() - 1 {
            return None;
        }

        match self.entries[idx].clone() {
            ModuleFormEntry::ValForm(form) => Some(form),
            _ => None,
        }
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

    pub fn entries_to_string(&self) -> String {
        let len = self.entries.len();

        match len {
            1 => self.entries[0].to_string(),
            _ => format!(
                "(prod {})",
                self.entries
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }

    fn parse_type_params(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.params[idx].clone() {
            FormParam::Ignore => {
                self.type_params.push(ModuleFormTypeParam::Ignore);
            }
            FormParam::Empty => {
                self.type_params.push(ModuleFormTypeParam::Empty);
            }
            FormParam::TypeKeyword(keyword) => {
                self.type_params.push(ModuleFormTypeParam::Keyword(keyword));
            }
            FormParam::TypeSymbol(symbol) => {
                if is_qualified(&symbol) {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified symbol".into(),
                    }));
                }

                self.type_params.push(ModuleFormTypeParam::Symbol(symbol));
            }
            FormParam::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::TypeKeyword(keyword) => {
                                self.type_params.push(ModuleFormTypeParam::Keyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                if is_qualified(&symbol) {
                                    return Err(Error::Syntactic(SyntacticError {
                                        loc: form.loc(),
                                        desc: "expected an unqualified symbol".into(),
                                    }));
                                }

                                self.type_params.push(ModuleFormTypeParam::Symbol(symbol));
                            }
                            ProdFormValue::TypesForm(form) => {
                                self.type_params.push(ModuleFormTypeParam::Form(form));
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

    fn parse_entries(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.params[idx].clone() {
            FormParam::Empty => {
                self.entries.push(ModuleFormEntry::Empty);
            }
            FormParam::Form(form) => {
                let prod = ProdForm::from_form(&form)?;

                for value in prod.values {
                    match value {
                        ProdFormValue::ImportForm(form) => {
                            self.entries.push(ModuleFormEntry::ImportForm(form));
                        }
                        ProdFormValue::ExportForm(form) => {
                            self.entries.push(ModuleFormEntry::ExportForm(form));
                        }
                        ProdFormValue::AttrsForm(form) => {
                            self.entries.push(ModuleFormEntry::AttrsForm(form));
                        }
                        ProdFormValue::TypeForm(form) => {
                            self.entries.push(ModuleFormEntry::TypeForm(form));
                        }
                        ProdFormValue::SigForm(form) => {
                            self.entries.push(ModuleFormEntry::SigForm(form));
                        }
                        ProdFormValue::ValForm(form) => {
                            self.entries.push(ModuleFormEntry::ValForm(form));
                        }
                        _ => {
                            return Err(Error::Syntactic(SyntacticError {
                                loc: form.loc(),
                                desc: "expected a product of import, export or value forms".into(),
                            }));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected an empty literal or a product of value forms".into(),
                }));
            }
        }

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<ModuleForm> {
        if form.name != "module" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a module keyword".into(),
            }));
        }

        let len = form.params.len();

        if len < 2 || len > 3 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name, an optional product of type parameters and a product of values".into(),
            }));
        }

        let mut module = ModuleForm::new();
        module.tokens = form.tokens.clone();

        let name = form.params[0].to_string();

        if !is_value_symbol(&name) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a value symbol".into(),
            }));
        }

        module.name = name;

        match len {
            2 => {
                module.parse_entries(form, 1)?;
            }
            3 => {
                module.parse_type_params(form, 1)?;
                module.parse_entries(form, 2)?;
            }
            _ => unreachable!(),
        }

        Ok(module)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ModuleForm> {
        let form = Form::from_tokens(tokens)?;

        ModuleForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ModuleForm> {
        let tokens = Tokens::from_str(s)?;

        ModuleForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        if self.type_params.is_empty() {
            format!("(module {} {})", self.name, self.entries_to_string(),)
        } else {
            format!(
                "(module {} {} {})",
                self.name,
                self.type_params_to_string(),
                self.entries_to_string(),
            )
        }
    }
}

impl fmt::Display for ModuleForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn module_form_from_str() {
        use super::ModuleForm;
        use super::ModuleFormEntry;
        use super::ModuleFormTypeParam;

        let mut s = "(module x () ())";

        let mut res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.type_params, vec![ModuleFormTypeParam::Empty]);
        assert_eq!(form.type_params_to_string(), "()".to_string());
        assert_eq!(form.entries, vec![ModuleFormEntry::Empty]);
        assert_eq!(form.to_string(), s.to_string());

        s = "(module x ())";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert!(form.type_params.is_empty());
        assert_eq!(form.type_params_to_string(), "".to_string());
        assert_eq!(form.entries, vec![ModuleFormEntry::Empty]);
        assert_eq!(form.to_string(), s.to_string());

        s = "
        (module x (prod T E) (prod
            (type Result (Sum T E))
            
            (sig unwrap (Fun (Result T E) T))
            (val unwrap (fun res 
                (case res 
                    (match t id)
                    (match e panic))))
        ))";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.type_params_to_string(), "(prod T E)".to_string());
        assert_eq!(form.entries.len(), 3);
        assert_eq!(
            form.entries[0].to_string(),
            "(type Result (Sum T E))".to_string()
        );
        assert!(form.entry_as_import(0).is_none());
        assert!(form.entry_as_export(0).is_none());
        assert!(form.entry_as_type(0).is_some());
        assert!(form.entry_as_type(0).unwrap().is_types_form());
        assert!(form.entry_as_value(0).is_none());
        assert_eq!(
            form.entries[1].to_string(),
            "(sig unwrap (Fun (Result T E) T))".to_string()
        );
        assert!(form.entry_as_import(1).is_none());
        assert!(form.entry_as_export(1).is_none());
        assert!(form.entry_as_type(1).is_none());
        assert!(form.entry_as_signature(1).is_some());
        assert!(form.entry_as_signature(1).is_some());
        assert!(form.entry_as_value(1).is_none());
        assert_eq!(
            form.entries[2].to_string(),
            "(val unwrap (fun res (case res (match t id) (match e panic))))".to_string()
        );
        assert!(form.entry_as_import(2).is_none());
        assert!(form.entry_as_export(2).is_none());
        assert!(form.entry_as_value(2).is_some());
        assert!(form.entry_as_value(2).unwrap().is_value());
        assert!(form.entry_as_value(2).unwrap().is_function_form());

        s = "
        (module main () (prod
            (type StringErr String)
            (import x (prod String StringErr) (prod Result unwrap))
            (import std.io _ println)

            (sig main (Fun IO IO))
            (val main (fun io (let 
                (sig text String)
                (val text \"Hello, World!\")
                (println (prod io (unwrap text))))))
        ))";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "main".to_string());
        assert_eq!(form.type_params, vec![ModuleFormTypeParam::Empty]);
        assert_eq!(form.type_params_to_string(), "()".to_string());
        assert_eq!(form.entries.len(), 5);
        assert_eq!(
            form.entries[0].to_string(),
            "(type StringErr String)".to_string()
        );
        assert!(form.entry_as_import(0).is_none());
        assert!(form.entry_as_export(0).is_none());
        assert!(form.entry_as_type(0).is_some());
        assert!(form.entry_as_type(0).unwrap().is_type_keyword());
        assert!(form.entry_as_value(0).is_none());
        assert_eq!(
            form.entries[1].to_string(),
            "(import x (prod String StringErr) (prod Result unwrap))".to_string()
        );
        assert!(form.entry_as_import(1).is_some());
        assert!(form.entry_as_export(1).is_none());
        assert!(form.entry_as_value(1).is_none());
        assert_eq!(
            form.entries[2].to_string(),
            "(import std.io _ println)".to_string()
        );
        assert!(form.entry_as_import(2).is_some());
        assert!(form.entry_as_export(2).is_none());
        assert!(form.entry_as_value(2).is_none());

        s = "
        (module main (prod
            (type StringErr String)
            (import x (prod String StringErr) (prod Result unwrap))
            (import std.io _ println)

            (sig main (Fun IO IO))
            (val main (fun io (let
                (sig text String)
                (val text \"Hello, World!\")
                (println (prod io (unwrap text))))))
        ))";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "main".to_string());
        assert!(form.type_params.is_empty());
        assert_eq!(form.entries.len(), 5);
        assert_eq!(
            form.entries[0].to_string(),
            "(type StringErr String)".to_string()
        );
        assert_eq!(
            form.entries[1].to_string(),
            "(import x (prod String StringErr) (prod Result unwrap))".to_string()
        );
        assert_eq!(
            form.entries[2].to_string(),
            "(import std.io _ println)".to_string()
        );
    }
}
