use crate::error::{Error, SyntacticError};
use crate::form::block_form::{BlockForm, BlockFormEntry};
use crate::form::form::{Form, FormTailElement};
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModuleFormTypeParameter {
    Empty(SimpleValue),
    Symbol(SimpleValue),
}

impl Default for ModuleFormTypeParameter {
    fn default() -> ModuleFormTypeParameter {
        ModuleFormTypeParameter::Empty(SimpleValue::new())
    }
}

impl ModuleFormTypeParameter {
    pub fn file(&self) -> String {
        match self {
            ModuleFormTypeParameter::Empty(empty) => empty.file(),
            ModuleFormTypeParameter::Symbol(symbol) => symbol.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ModuleFormTypeParameter::Empty(empty) => empty.loc(),
            ModuleFormTypeParameter::Symbol(symbol) => symbol.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ModuleFormTypeParameter::Empty(_) => "()".into(),
            ModuleFormTypeParameter::Symbol(symbol) => symbol.to_string(),
        }
    }
}

impl fmt::Display for ModuleFormTypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ModuleFormBlock {
    Empty(SimpleValue),
    Form(Box<BlockForm>),
}

impl Default for ModuleFormBlock {
    fn default() -> ModuleFormBlock {
        ModuleFormBlock::Empty(SimpleValue::new())
    }
}

impl ModuleFormBlock {
    pub fn file(&self) -> String {
        match self {
            ModuleFormBlock::Empty(empty) => empty.file(),
            ModuleFormBlock::Form(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            ModuleFormBlock::Empty(empty) => empty.loc(),
            ModuleFormBlock::Form(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ModuleFormBlock::Empty(_) => "()".into(),
            ModuleFormBlock::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ModuleFormBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ModuleForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub type_parameters: Vec<ModuleFormTypeParameter>,
    pub block: ModuleFormBlock,
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

    pub fn block_entries(&self) -> Vec<BlockFormEntry> {
        let mut entries = vec![];

        match self.block.clone() {
            ModuleFormBlock::Empty(_) => {}
            ModuleFormBlock::Form(form) => {
                entries = form.entries;
            }
        }

        entries
    }

    pub fn type_parameters_to_string(&self) -> String {
        match self.type_parameters.len() {
            1 => self.type_parameters[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.type_parameters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "".to_string(),
        }
    }

    fn parse_type_parameters(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    self.type_parameters
                        .push(ModuleFormTypeParameter::Empty(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    self.type_parameters
                        .push(ModuleFormTypeParameter::Symbol(value));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected an unqualified type symbol or an empty literal".into(),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::TypeSymbol(symbol) => {
                                self.type_parameters
                                    .push(ModuleFormTypeParameter::Symbol(symbol));
                            }
                            x => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: x.loc(),
                                    desc: "expected an unqualified type symbol".into(),
                                }));
                            }
                        }
                    }
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a product of types".into(),
                    }));
                }
            }
        }

        Ok(())
    }

    fn parse_block(&mut self, form: &Form, idx: usize) -> Result<()> {
        match form.tail[idx].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    self.block = ModuleFormBlock::Empty(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                let form = BlockForm::from_form(&form)?;
                self.block = ModuleFormBlock::Form(Box::new(form));
            }
        }

        Ok(())
    }

    pub fn from_form(form: &Form) -> Result<ModuleForm> {
        if form.head.to_string() != "module" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a module keyword".into(),
            }));
        }

        let len = form.tail.len();

        if len < 2 || len > 3 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name, optional type parameters and a product of forms".into(),
            }));
        }

        let mut module = ModuleForm::new();
        module.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    module.name = value;
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected an unqualified value symbol".into(),
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

        match len {
            2 => {
                module.parse_block(form, 1)?;
            }
            3 => {
                module.parse_type_parameters(form, 1)?;
                module.parse_block(form, 2)?;
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
        if self.type_parameters.is_empty() {
            format!("(module {} {})", self.name, self.block.to_string(),)
        } else {
            format!(
                "(module {} {} {})",
                self.name,
                self.type_parameters_to_string(),
                self.block.to_string(),
            )
        }
    }
}

impl fmt::Display for ModuleForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for ModuleForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn module_form_from_str() {
        use super::ModuleForm;

        let mut s = "(module x () ())";

        let mut res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert_eq!(form.type_parameters_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(module x ())";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert!(form.type_parameters.is_empty());
        assert_eq!(form.type_parameters_to_string(), "".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "
        (module x (prod T E) (block
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

        assert_eq!(form.name.to_string(), "x".to_string());
        assert_eq!(form.type_parameters_to_string(), "(prod T E)".to_string());

        let mut block_entries = form.block_entries();

        assert_eq!(form.block_entries().len(), 3);
        assert_eq!(
            block_entries[0].to_string(),
            "(type Result (Sum T E))".to_string()
        );
        assert_eq!(
            block_entries[1].to_string(),
            "(sig unwrap (Fun (Result T E) T))".to_string()
        );
        assert_eq!(
            block_entries[2].to_string(),
            "(val unwrap (fun res (case res (match t id) (match e panic))))".to_string()
        );

        s = "
        (module main () (block
            (type StringErr String)
            (import x (prod String StringErr) (prod Result unwrap))
            (import std.io _ println)

            (sig main (Fun IO IO))
            (val main (fun io (let 
                (sig text String)
                (val text \"Hello, World!\")
                (println io (unwrap text)))))))";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "main".to_string());
        assert_eq!(form.type_parameters_to_string(), "()".to_string());

        block_entries = form.block_entries();

        assert_eq!(block_entries.len(), 5);
        assert_eq!(
            block_entries[0].to_string(),
            "(type StringErr String)".to_string()
        );
        assert_eq!(
            block_entries[1].to_string(),
            "(import x (prod String StringErr) (prod Result unwrap))".to_string()
        );
        assert_eq!(
            block_entries[2].to_string(),
            "(import std.io _ println)".to_string()
        );

        s = "
        (module main (block
            (type StringErr String)
            (import x (prod String StringErr) (prod Result unwrap))
            (import std.io _ println)

            (sig main (Fun IO IO))
            (val main (fun io (let
                (sig text String)
                (val text \"Hello, World!\")
                (println io (unwrap text)))))))";

        res = ModuleForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "main".to_string());
        assert!(form.type_parameters.is_empty());

        block_entries = form.block_entries();

        assert_eq!(block_entries.len(), 5);
        assert_eq!(
            block_entries[0].to_string(),
            "(type StringErr String)".to_string()
        );
        assert_eq!(
            block_entries[1].to_string(),
            "(import x (prod String StringErr) (prod Result unwrap))".to_string()
        );
        assert_eq!(
            block_entries[2].to_string(),
            "(import std.io _ println)".to_string()
        );
    }
}
