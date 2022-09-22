use crate::chunks::StringChunks;
use crate::error::{Error, SyntaxError};
use crate::result::Result;
use crate::syntax::is_keyword;
use crate::syntax::SINGLE_QUOTE;
use crate::syntax::{is_comment_mark, is_doc_comment_mark};
use crate::syntax::{is_double_quote, is_single_quote};
use crate::syntax::{is_escape_char, is_whitespace};
use crate::syntax::{is_float_literal, is_int_literal, is_uint_literal};
use crate::syntax::{is_form_end, is_form_start};
use crate::syntax::{is_path_symbol, is_symbol, is_type_symbol, is_value_symbol};
use crate::token::Token;
use std::convert;
use std::fs;
use std::iter;
use std::ops;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Tokens(Vec<Token>);

impl Tokens {
    pub fn new() -> Self {
        Tokens::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let chunks = StringChunks::from_str(s);
        let len = chunks.len();
        let mut idx = 0;

        let mut forms_count = 0;
        let mut open_form_idxs = vec![idx];
        let mut close_form_idxs = vec![];

        let mut tokens = Tokens::new();

        while idx < len {
            let chunk = chunks[idx].clone();
            let s = chunk.content.clone();

            match s.clone() {
                mut x if (is_comment_mark(&x) || is_doc_comment_mark(&x)) => {
                    let mut token = if is_comment_mark(&x) {
                        Token::new_comment()
                    } else {
                        Token::new_doc_comment()
                    };

                    let mut cchunk = chunk;

                    if idx + 1 >= len {
                        token.push(cchunk);
                        tokens.push(token);
                        break;
                    }

                    idx += 1;

                    x = chunks[idx].content.clone();

                    while idx < len {
                        cchunk.content.push_str(&x);

                        if x == '\n'.to_string() {
                            break;
                        }

                        if idx + 1 >= len {
                            break;
                        }

                        idx += 1;

                        x = chunks[idx].content.clone();
                    }

                    token.push(cchunk.clone());
                    tokens.push(token);

                    idx += 1;
                }
                x if is_keyword(&x) => {
                    let mut token = Token::new_keyword();
                    token.push(chunk.clone());

                    idx += 1;

                    tokens.push(token);
                }
                x if is_uint_literal(&x) => {
                    let mut token = Token::new_uint_literal();
                    token.push(chunk.clone());

                    idx += 1;

                    tokens.push(token);
                }
                x if is_int_literal(&x) => {
                    let mut token = Token::new_int_literal();
                    token.push(chunk.clone());

                    idx += 1;

                    tokens.push(token);
                }
                x if is_float_literal(&x) => {
                    let mut token = Token::new_float_literal();
                    token.push(chunk.clone());

                    idx += 1;

                    tokens.push(token);
                }
                x if is_single_quote(&x) => {
                    if idx + 2 > len {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[idx].loc.clone()),
                            desc: "expected a char".into(),
                        }));
                    }

                    let mut schunk = chunk;
                    let mut rem_len = 2;

                    while rem_len > 0 {
                        idx += 1;

                        let c = chunks[idx].content.clone();
                        if c.len() != 1 {
                            return Err(Error::Syntax(SyntaxError {
                                loc: Some(chunks[idx].loc.clone()),
                                desc: "expected a char".into(),
                            }));
                        }

                        if is_escape_char(&c)
                            && idx + 2 < len
                            && (is_single_quote(&chunks[idx + 1].content)
                                || is_escape_char(&chunks[idx + 1].content))
                        {
                            continue;
                        }

                        schunk.content.push_str(&c);

                        rem_len -= 1;
                    }

                    if !schunk.content.ends_with(SINGLE_QUOTE) {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[idx].loc.clone()),
                            desc: "expected a char".into(),
                        }));
                    }

                    let mut token = Token::new_char_literal();
                    token.push(schunk);

                    tokens.push(token);

                    idx += 1;
                }
                mut x if is_double_quote(&x) => {
                    let mut schunk = chunk;

                    if idx + 1 >= len {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[idx].loc.clone()),
                            desc: "expected a string".into(),
                        }));
                    }

                    idx += 1;

                    while idx < len {
                        x = chunks[idx].content.clone();
                        schunk.content.push_str(&x);

                        if is_escape_char(&x)
                            && idx + 2 < len
                            && (is_double_quote(&chunks[idx + 1].content)
                                || is_escape_char(&chunks[idx + 1].content))
                        {
                            idx += 1;
                            x = chunks[idx].content.clone();
                            schunk.content.push_str(&x);
                            idx += 1;
                            continue;
                        }

                        if is_double_quote(&x) {
                            break;
                        }

                        idx += 1;
                    }

                    let mut token = Token::new_string_literal();
                    token.push(schunk.clone());
                    tokens.push(token);

                    idx += 1;
                }
                mut x if is_form_start(&x) => {
                    forms_count += 1;
                    open_form_idxs.push(idx);
                    let mut is_empty = false;

                    if idx + 1 < len && is_form_end(&chunks[idx + 1].content) {
                        is_empty = true;
                    }

                    let mut token = if is_empty {
                        Token::new_empty_literal()
                    } else {
                        Token::new_form_start()
                    };

                    if is_empty {
                        let mut fchunk = chunk;

                        idx += 1;

                        forms_count -= 1;
                        open_form_idxs.pop();
                        close_form_idxs.push(idx);

                        x = chunks[idx].content.clone();
                        fchunk.content.push_str(&x);

                        token.push(fchunk);
                        tokens.push(token);

                        idx += 1;
                    } else {
                        token.push(chunk.clone());
                        tokens.push(token);
                        idx += 1;
                    }
                }
                x if is_form_end(&x) => {
                    forms_count -= 1;

                    if forms_count < 0 {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[idx].loc.clone()),
                            desc: "closing a form never opened".into(),
                        }));
                    }

                    open_form_idxs.pop();
                    close_form_idxs.push(idx);

                    let mut token = Token::new_form_end();
                    token.push(chunk.clone());

                    tokens.push(token);

                    idx += 1;
                }
                x if is_symbol(&x) => {
                    let mut token = if is_type_symbol(&x) {
                        Token::new_type_symbol()
                    } else if is_value_symbol(&x) {
                        Token::new_value_symbol()
                    } else if is_path_symbol(&x) {
                        Token::new_path_symbol()
                    } else {
                        panic!("expected a symbol");
                    };

                    token.push(chunk.clone());

                    tokens.push(token);

                    idx += 1;
                }
                x if is_whitespace(&x) => {
                    idx += 1;
                }
                _ => {
                    println!("idx: {}, s: {}", idx, s);
                    return Err(Error::Syntax(SyntaxError {
                        loc: Some(chunks[idx].loc.clone()),
                        desc: "unrecognized syntax".into(),
                    }));
                }
            }
        }

        if forms_count != 0 {
            let (err_idx, desc): (usize, String) = if forms_count > 0 {
                (open_form_idxs.remove(0), "form not closed".into())
            } else {
                (
                    close_form_idxs.pop().unwrap(),
                    "closing a form never opened".into(),
                )
            };

            return Err(Error::Syntax(SyntaxError {
                loc: Some(chunks[err_idx].loc.clone()),
                desc,
            }));
        }

        Ok(tokens)
    }

    pub fn from_string(s: String) -> Result<Self> {
        Self::from_str(&s)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_string(fs::read_to_string(path)?)
    }
}

impl ops::Index<usize> for Tokens {
    type Output = Token;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl iter::IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl iter::FromIterator<Token> for Tokens {
    fn from_iter<I: iter::IntoIterator<Item = Token>>(iter: I) -> Self {
        let mut tokens = Tokens::new();

        for token in iter {
            tokens.push(token);
        }

        tokens
    }
}

impl convert::From<Vec<Token>> for Tokens {
    fn from(tokens: Vec<Token>) -> Self {
        Tokens(tokens)
    }
}

impl std::str::FromStr for Tokens {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Tokens::from_str(s)
    }
}

impl convert::TryFrom<String> for Tokens {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Tokens::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn comment_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "# this is a comment\n# this is an other comment\n\t# this is an other\t comment\n";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2].kind, TokenKind::Comment);
    }

    #[test]
    fn doc_comment_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s =
            "#! this is a comment\n#! this is an other comment\n\t#! this is an other\t comment\n";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[2].kind, TokenKind::DocComment);
    }

    #[test]
    fn keyword_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "defsum";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
    }

    #[test]
    fn empty_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "()";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::EmptyLiteral);
    }

    #[test]
    fn uint_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "XFFF";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::UIntLiteral);
    }

    #[test]
    fn int_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "-o476";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::IntLiteral);
    }

    #[test]
    fn float_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "-0.1E-10";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::FloatLiteral);
    }

    #[test]
    fn char_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "'\\'";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);

        for token in tokens.into_iter() {
            assert_eq!(token.kind, TokenKind::CharLiteral);
        }
    }

    #[test]
    fn string_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "\"\\\"";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn symbol_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let mut s = "aB0c-d.,ef_!+/9";

        let mut res = Tokens::from_str(s);

        assert!(res.is_err());

        s = "_|2a";

        res = Tokens::from_str(s);

        assert!(res.is_err());

        s = "<=>";

        let mut tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::ValueSymbol);

        s = "BigInt";

        tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::TypeSymbol);

        s = "a.b.c";

        tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::PathSymbol);
    }

    #[test]
    fn forms_tokens() {
        use super::Tokens;
        use crate::error::{Error, SyntaxError};
        use crate::token::TokenKind;

        let mut s = "()))";

        let mut res = Tokens::from_str(s);

        assert!(res.is_err());

        match res {
            Err(Error::Syntax(SyntaxError { loc, desc })) => {
                assert_eq!(loc.unwrap().pos, 2);

                let err_desc: String = "closing a form never opened".into();
                assert_eq!(desc, err_desc);
            }
            _ => panic!("invalid branch"),
        }

        s = "((( ( () ) () )) (";

        res = Tokens::from_str(s);

        assert!(res.is_err());

        match res {
            Err(Error::Syntax(SyntaxError { loc, desc })) => {
                assert_eq!(loc.unwrap().pos, 0);

                let err_desc: String = "form not closed".into();
                assert_eq!(desc, err_desc);
            }
            _ => panic!("invalid branch"),
        }

        s = "(defun f b c \n\t\t(g a -b1\n\t\t\t\t(h 3 b +5.8E+36) # this is a comment\n\t\t\t\t(k 6 7 c)))\n(f 1 4 8)\n";

        res = Tokens::from_str(s);

        assert!(res.is_ok());

        let tokens = res.unwrap();

        assert_eq!(tokens.len(), 30);
        assert_eq!(tokens[0].kind, TokenKind::FormStart);
        assert_eq!(tokens[1].kind, TokenKind::Keyword);
        assert_eq!(tokens[2].kind, TokenKind::ValueSymbol);
        assert_eq!(tokens[8].kind, TokenKind::IntLiteral);
        assert_eq!(tokens[11].kind, TokenKind::UIntLiteral);
        assert_eq!(tokens[13].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[14].kind, TokenKind::FormEnd);
        assert_eq!(tokens[15].kind, TokenKind::Comment);
    }

    #[test]
    fn tokens_from_file() {
        use super::Tokens;
        use crate::token::TokenKind;
        use std::path::Path;

        let path = Path::new("./examples/hello_world_2.sp");

        let res = Tokens::from_file(path);

        if res.is_err() {
            res.as_ref().unwrap();
        }

        assert!(res.is_ok());

        let tokens = res.unwrap();

        assert_eq!(tokens.len(), 21);
        assert_eq!(tokens[0].kind, TokenKind::DocComment);
        assert_eq!(tokens[1].kind, TokenKind::FormStart);
        assert_eq!(tokens[2].kind, TokenKind::Keyword);
        assert_eq!(tokens[3].kind, TokenKind::PathSymbol);
        assert_eq!(tokens[17].kind, TokenKind::CharLiteral);
        assert_eq!(
            tokens[17].chunks.as_ref().unwrap()[0].content,
            "'''".to_string()
        );
        assert_eq!(tokens[18].kind, TokenKind::StringLiteral);
    }
}
