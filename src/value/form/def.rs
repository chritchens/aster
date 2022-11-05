use super::{AttrsForm, FunForm, SigForm, TypeForm};
use super::{FunAppForm, TypeAppForm};
use super::{MixedAppForm, MixedAppFormParam};
use super::{PrimForm, SumForm};
use super::{SymbolProdForm, TypeProdForm, ValueProdForm};
use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DefFormValue {
    Empty,
    Prim(String),
    ValueSymbol(String),
    TypeSymbol(String),
    PrimForm(PrimForm),
    SumForm(SumForm),
    SymbolProdForm(SymbolProdForm),
    ValueProdForm(ValueProdForm),
    TypeProdForm(TypeProdForm),
    FunForm(FunForm),
    TypeForm(TypeForm),
    SigForm(SigForm),
    AttrsForm(AttrsForm),
    FunAppForm(FunAppForm),
    TypeAppForm(TypeAppForm),
}

impl Default for DefFormValue {
    fn default() -> DefFormValue {
        DefFormValue::Empty
    }
}

impl DefFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            DefFormValue::Empty => "()".into(),
            DefFormValue::Prim(prim) => prim.clone(),
            DefFormValue::ValueSymbol(symbol) => symbol.clone(),
            DefFormValue::TypeSymbol(symbol) => symbol.clone(),
            DefFormValue::PrimForm(prim_form) => prim_form.to_string(),
            DefFormValue::SumForm(sum_form) => sum_form.to_string(),
            DefFormValue::SymbolProdForm(symbol_prod_form) => symbol_prod_form.to_string(),
            DefFormValue::ValueProdForm(value_prod_form) => value_prod_form.to_string(),
            DefFormValue::TypeProdForm(type_prod_form) => type_prod_form.to_string(),
            DefFormValue::FunForm(fun_form) => fun_form.to_string(),
            DefFormValue::TypeForm(type_form) => type_form.to_string(),
            DefFormValue::SigForm(sig_form) => sig_form.to_string(),
            DefFormValue::AttrsForm(attrs_form) => attrs_form.to_string(),
            DefFormValue::FunAppForm(fun_app_form) => fun_app_form.to_string(),
            DefFormValue::TypeAppForm(type_app_form) => type_app_form.to_string(),
        }
    }
}

impl fmt::Display for DefFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct DefForm {
    pub tokens: Tokens,
    pub name: String,
    pub value: DefFormValue,
}

impl DefForm {
    pub fn new() -> DefForm {
        DefForm::default()
    }

    pub fn is_empty_primitive(&self) -> bool {
        match self.value {
            DefFormValue::Empty => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self.value {
            DefFormValue::Prim(_) => true,
            _ => false,
        }
    }

    pub fn is_value_symbol(&self) -> bool {
        match self.value {
            DefFormValue::ValueSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_type_symbol(&self) -> bool {
        match self.value {
            DefFormValue::TypeSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_primitive_form(&self) -> bool {
        match self.value {
            DefFormValue::PrimForm(_) => true,
            _ => false,
        }
    }

    pub fn is_sum_form(&self) -> bool {
        match self.value {
            DefFormValue::SumForm(_) => true,
            _ => false,
        }
    }

    pub fn is_symbols_product_form(&self) -> bool {
        match self.value {
            DefFormValue::SymbolProdForm(_) => true,
            _ => false,
        }
    }

    pub fn is_values_product_form(&self) -> bool {
        match self.value {
            DefFormValue::ValueProdForm(_) => true,
            _ => false,
        }
    }

    pub fn is_types_product_form(&self) -> bool {
        match self.value {
            DefFormValue::TypeProdForm(_) => true,
            _ => false,
        }
    }

    pub fn is_function_form(&self) -> bool {
        match self.value {
            DefFormValue::FunForm(_) => true,
            _ => false,
        }
    }

    pub fn is_type_form(&self) -> bool {
        match self.value {
            DefFormValue::TypeForm(_) => true,
            _ => false,
        }
    }

    pub fn is_signature_form(&self) -> bool {
        match self.value {
            DefFormValue::SigForm(_) => true,
            _ => false,
        }
    }

    pub fn is_attributes_form(&self) -> bool {
        match self.value {
            DefFormValue::AttrsForm(_) => true,
            _ => false,
        }
    }

    pub fn is_function_application_form(&self) -> bool {
        match self.value {
            DefFormValue::FunAppForm(_) => true,
            _ => false,
        }
    }

    pub fn is_type_application_form(&self) -> bool {
        match self.value {
            DefFormValue::TypeAppForm(_) => true,
            _ => false,
        }
    }

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<DefForm> {
        if mixed_app.name != "def" {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a def keyword".into(),
            }));
        }

        if mixed_app.params.len() != 2 {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a name and a form or a symbol or a primitive".into(),
            }));
        }

        let mut def = DefForm::new();
        def.tokens = mixed_app.tokens.clone();

        match mixed_app.params[0].clone() {
            MixedAppFormParam::ValueSymbol(name) => {
                def.name = name;
            }
            MixedAppFormParam::TypeSymbol(name) => {
                def.name = name;
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a value symbol or a type symbol".into(),
                }));
            }
        }

        match mixed_app.params[1].clone() {
            MixedAppFormParam::Empty => {
                def.value = DefFormValue::Empty;
            }
            MixedAppFormParam::Prim(prim) => {
                def.value = DefFormValue::Prim(prim);
            }
            MixedAppFormParam::ValueSymbol(symbol) => {
                def.value = DefFormValue::ValueSymbol(symbol);
            }
            MixedAppFormParam::TypeSymbol(symbol) => {
                def.value = DefFormValue::TypeSymbol(symbol);
            }
            MixedAppFormParam::FunApp(fun_app) => match fun_app.name.as_str() {
                "prim" => {
                    let form = PrimForm::from_fun_app(&fun_app)?;
                    def.value = DefFormValue::PrimForm(form);
                }
                "sum" => {
                    let form = SumForm::from_fun_app(&fun_app)?;
                    def.value = DefFormValue::SumForm(form);
                }
                "attrs" => {
                    let form = AttrsForm::from_fun_app(&fun_app)?;
                    def.value = DefFormValue::AttrsForm(form);
                }
                "fun" => {
                    let form = FunForm::from_fun_app(&fun_app)?;
                    def.value = DefFormValue::FunForm(form);
                }
                "prod" => {
                    if let Ok(form) = SymbolProdForm::from_fun_app(&fun_app) {
                        def.value = DefFormValue::SymbolProdForm(form);
                    } else {
                        let form = ValueProdForm::from_fun_app(&fun_app)?;
                        def.value = DefFormValue::ValueProdForm(form);
                    }
                }
                _ => {
                    def.value = DefFormValue::FunAppForm(fun_app);
                }
            },
            MixedAppFormParam::TypeApp(type_app) => {
                def.value = DefFormValue::TypeAppForm(type_app);
            }
            MixedAppFormParam::MixedApp(mixed_app) => match mixed_app.name.as_str() {
                "type" => {
                    let form = TypeForm::from_mixed_app(&mixed_app)?;
                    def.value = DefFormValue::TypeForm(form);
                }
                "sig" => {
                    let form = SigForm::from_mixed_app(&mixed_app)?;
                    def.value = DefFormValue::SigForm(form);
                }
                "prod" => {
                    if let Ok(form) = SymbolProdForm::from_mixed_app(&mixed_app) {
                        def.value = DefFormValue::SymbolProdForm(form);
                    } else {
                        let form = TypeProdForm::from_mixed_app(&mixed_app)?;
                        def.value = DefFormValue::TypeProdForm(form);
                    }
                }
                x => {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: format!("unexpected {} form", x),
                    }));
                }
            },
        }

        Ok(def)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<DefForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        DefForm::from_mixed_app(&mixed_app)
    }

    pub fn from_str(s: &str) -> Result<DefForm> {
        let tokens = Tokens::from_str(s)?;

        DefForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(def {} {})", self.name, self.value.to_string())
    }
}

impl fmt::Display for DefForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn def_form_from_str() {
        use super::DefForm;

        let mut s = "(def empty ())";

        let mut res = DefForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "empty".to_string());
        assert_eq!(form.value.to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_empty_primitive());

        s = "(def x 10)";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_primitive());

        s = "(def w x)";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "w".to_string());
        assert_eq!(form.value.to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_value_symbol());

        s = "(def s (sum a))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "s".to_string());
        assert_eq!(form.value.to_string(), "(sum a)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_sum_form());

        s = "(def p (prod a b c d))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "p".to_string());
        assert_eq!(form.value.to_string(), "(prod a b c d)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_symbols_product_form());

        s = "(def p (prod a b (f x y 10) 11))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "p".to_string());
        assert_eq!(
            form.value.to_string(),
            "(prod a b (f x y 10) 11)".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_values_product_form());

        s = "(def C Char)";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "C".to_string());
        assert_eq!(form.value.to_string(), "Char".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_type_symbol());

        s = "(def Result (type (prod T E) (Sum T E)))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Result".to_string());
        assert_eq!(
            form.value.to_string(),
            "(type (prod T E) (Sum T E))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_type_form());

        s = "(def newSum (sig (Fun (Prod Int Int) Int)))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "newSum".to_string());
        assert_eq!(
            form.value.to_string(),
            "(sig (Fun (Prod Int Int) Int))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_signature_form());

        s = "(def newSum (attrs (prod attr1 attr2)))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "newSum".to_string());
        assert_eq!(
            form.value.to_string(),
            "(attrs (prod attr1 attr2))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_attributes_form());

        s = "(def newSum (fun (prod a b) (+ a b)))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "newSum".to_string());
        assert_eq!(
            form.value.to_string(),
            "(fun (prod a b) (+ a b))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_function_form());

        s = "(def y (app f a b c d))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "y".to_string());
        assert_eq!(form.value.to_string(), "(app f a b c d)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_function_application_form());

        s = "(def Y (Fun (Prod A B C) D))";

        res = DefForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Y".to_string());
        assert_eq!(form.value.to_string(), "(Fun (Prod A B C) D)".to_string());
        assert_eq!(form.to_string(), s.to_string());
        assert!(form.is_type_application_form());
    }
}
