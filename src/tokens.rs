use crate::chunks::Chunks;
use crate::error::{Error, SyntaxError};
use crate::keyword::KEYWORDS;
use crate::keyword::{COMMENT_MARK, COMMENT_MARK_POSTFIX};
use crate::keyword::{FORM_END, FORM_START};
use crate::result::Result;
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_string(fs::read_to_string(path)?)
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
        let chunks = Chunks::from_str(s);
        let len = chunks.len();
        let mut idx = 0;

        let mut forms_count = 0;
        let mut open_form_idxs = vec![idx];
        let mut close_form_idxs = vec![];

        let mut tokens = Tokens::new();

        while idx != len {
            let mut chunk = chunks[idx].clone();

            let mut c = chunk.content;

            match c {
                COMMENT_MARK => {
                    let next_chunk = if idx != len - 1 {
                        Some(chunks[idx + 1].clone())
                    } else {
                        None
                    };

                    let mut token = Token::new_comment();

                    if let Some(nxtc) = next_chunk {
                        if nxtc.content == COMMENT_MARK_POSTFIX {
                            token = Token::new_doc_comment()
                        }
                    };

                    while idx < len && c != '\n' {
                        c = chunk.content;
                        token.push(chunk.clone());

                        idx += 1;

                        if idx == len {
                            break;
                        }

                        chunk = chunks[idx].clone();
                    }

                    tokens.push(token);
                }
                'i' | 'd' => {
                    let mut token = Token::new_keyword();
                    let mut is_keyword = false;
                    let mut klen = 0;

                    for keyword in KEYWORDS.iter() {
                        klen = keyword.len();

                        if idx + klen > len {
                            continue;
                        }

                        let chars: String = (0..klen).map(|i| chunks[idx + i].content).collect();

                        if keyword != &chars {
                            continue;
                        }

                        for i in 0..klen {
                            token.push(chunks[idx + i].clone());
                        }

                        is_keyword = true;

                        tokens.push(token);

                        break;
                    }

                    if is_keyword {
                        idx += klen;
                    } else {
                        idx += 1;
                    }
                }
                '0'..='9' | 'b' | 'o' | 'x' | '+' | '-' => {
                    let mut is_binary = false;
                    let mut is_octal = false;
                    let mut is_decimal = false;
                    let mut is_hexa = false;
                    let mut is_uint = false;
                    let mut is_int = false;
                    let mut is_float = false;
                    let mut is_symbol = false;
                    let mut sign_idx = None;
                    let mut radix_idx = None;
                    let mut point_idx = None;
                    let mut exp_idx = None;

                    let start_idx = idx;
                    let mut num_chunks = vec![];

                    while idx < len {
                        match c {
                            'b' | 'o' | 'x' => {
                                if idx != start_idx
                                    && (sign_idx.is_none()
                                        || (idx != start_idx + 1 && idx + 1 == len))
                                {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a well-formed number".into(),
                                    }));
                                }

                                radix_idx = Some(idx);

                                match c {
                                    'b' => is_binary = true,
                                    'o' => is_octal = true,
                                    'x' => is_hexa = true,
                                    _ => unreachable!(),
                                }

                                if idx + 1 == len || chunks[idx + 1].content == ' ' {
                                    let mut token = Token::new_symbol();
                                    token.push(chunk.clone());
                                    tokens.push(token);

                                    is_symbol = true;

                                    break;
                                }

                                num_chunks.push(chunk.clone());

                                idx += 1;
                                chunk = chunks[idx].clone();
                                c = chunk.content;
                            }
                            '+' => {
                                if sign_idx.is_some() && idx > start_idx && exp_idx != Some(idx - 1)
                                {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a float".into(),
                                    }));
                                }

                                if idx + 1 == len || chunks[idx + 1].content == ' ' {
                                    let mut token = Token::new_symbol();
                                    token.push(chunk.clone());
                                    tokens.push(token);

                                    is_symbol = true;

                                    break;
                                }

                                if idx == start_idx {
                                    sign_idx = Some(idx);
                                    is_int = true;
                                }

                                num_chunks.push(chunk.clone());

                                idx += 1;
                                chunk = chunks[idx].clone();
                                c = chunk.content;
                            }
                            '-' => {
                                if sign_idx.is_some() && idx > start_idx && exp_idx != Some(idx - 1)
                                {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a float".into(),
                                    }));
                                }

                                if idx + 1 == len || chunks[idx + 1].content == ' ' {
                                    let mut token = Token::new_symbol();
                                    token.push(chunk.clone());
                                    tokens.push(token);

                                    is_symbol = true;

                                    break;
                                }

                                if idx == start_idx {
                                    sign_idx = Some(idx);
                                    is_int = true;
                                }

                                num_chunks.push(chunk.clone());

                                idx += 1;
                                chunk = chunks[idx].clone();
                                c = chunk.content;
                            }
                            'E' => {
                                if (exp_idx != None && idx != start_idx) || !is_float {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a float".into(),
                                    }));
                                }

                                if idx == start_idx && idx + 1 == len
                                    || chunks[idx + 1].content == ' '
                                {
                                    let mut token = Token::new_symbol();
                                    token.push(chunk.clone());
                                    tokens.push(token);

                                    is_symbol = true;

                                    break;
                                }

                                exp_idx = Some(idx);

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            '0' => {
                                match idx {
                                    x if x == start_idx => {
                                        if idx + 1 < len {
                                            if chunks[idx + 1].content != '.' {
                                                return Err(Error::Syntax(SyntaxError {
                                                    loc: Some(chunks[idx].loc.clone()),
                                                    desc: "expected a float".into(),
                                                }));
                                            } else {
                                                is_float = true;
                                                is_uint = false;
                                                is_int = false;
                                            }
                                        } else {
                                            is_uint = true;
                                            is_decimal = true;
                                        }
                                    }
                                    x if x > start_idx => {
                                        if (sign_idx == Some(idx - 1) && idx + 1 == len)
                                            || point_idx == Some(idx - 1)
                                            || exp_idx == Some(idx - 1)
                                        {
                                            return Err(Error::Syntax(SyntaxError {
                                                loc: Some(chunks[idx].loc.clone()),
                                                desc: "expected a number".into(), // +/-0, bn.0, E0 are not valid numbers
                                            }));
                                        }

                                        if radix_idx == Some(idx - 1)
                                            && idx + 1 < len
                                            && chunks[idx + 1].content != ' '
                                        {
                                            return Err(Error::Syntax(SyntaxError {
                                                loc: Some(chunks[idx].loc.clone()),
                                                desc: "expected zero alone or a number greater than zero".into(),
                                            }));
                                        }

                                        if point_idx.is_none() {
                                            if sign_idx.is_some() {
                                                is_int = true;
                                            } else {
                                                is_uint = true;
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                }

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            '1' => {
                                if !is_binary && !is_octal && !is_decimal && !is_hexa {
                                    is_decimal = true;
                                }

                                if idx == start_idx
                                    || (radix_idx == Some(start_idx) && idx == start_idx + 1)
                                    || (!is_int && !is_float)
                                {
                                    is_uint = true;
                                }

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            '2'..='7' => {
                                if is_binary {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a binary number".into(),
                                    }));
                                }

                                if !is_octal && !is_decimal && !is_hexa {
                                    is_decimal = true;
                                }

                                if idx == start_idx
                                    || (radix_idx == Some(start_idx) && idx == start_idx + 1)
                                    || (!is_int && !is_float)
                                {
                                    is_uint = true;
                                }

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            '8'..='9' => {
                                if is_binary {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a binary number".into(),
                                    }));
                                }

                                if is_octal {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected an octal number".into(),
                                    }));
                                }

                                if !is_decimal {
                                    is_decimal = true;
                                }

                                if idx == start_idx
                                    || (radix_idx == Some(start_idx) && idx == start_idx + 1)
                                    || (!is_int && !is_float)
                                {
                                    is_uint = true;
                                }

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            'A'..='F' | 'a'..='f' => {
                                if is_binary {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a binary number".into(),
                                    }));
                                }

                                if is_octal {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected an octal number".into(),
                                    }));
                                }

                                if is_decimal {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a decimal number".into(),
                                    }));
                                }

                                if idx == start_idx
                                    || (radix_idx == Some(start_idx) && idx == start_idx + 1)
                                    || (!is_int && !is_float)
                                {
                                    is_uint = true;
                                }

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            '.' => {
                                if point_idx != None && idx != start_idx {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[idx].loc.clone()),
                                        desc: "expected a float".into(),
                                    }));
                                }

                                if idx == start_idx && idx + 1 == len
                                    || chunks[idx + 1].content == ' '
                                {
                                    let mut token = Token::new_symbol();
                                    token.push(chunk.clone());
                                    tokens.push(token);

                                    is_symbol = true;

                                    break;
                                }

                                point_idx = Some(idx);

                                is_float = true;
                                is_uint = false;
                                is_int = false;

                                num_chunks.push(chunk.clone());

                                if idx + 1 < len {
                                    idx += 1;
                                    chunk = chunks[idx].clone();
                                    c = chunk.content;
                                } else {
                                    break;
                                }
                            }
                            w if !w.is_ascii_alphanumeric()
                                && (w == COMMENT_MARK
                                    || w == FORM_START
                                    || w == FORM_END
                                    || w.is_whitespace()) =>
                            {
                                idx -= 1; // can't be tokenized as number
                                break;
                            }
                            _ => {
                                return Err(Error::Syntax(SyntaxError {
                                    loc: Some(chunks[start_idx].loc.clone()),
                                    desc: "expected a number or a symbol".into(),
                                }));
                            }
                        }
                    }

                    if !is_symbol {
                        let mut token = if is_uint {
                            Token::new_uint_literal()
                        } else if is_int {
                            Token::new_int_literal()
                        } else if is_float {
                            Token::new_float_literal()
                        } else {
                            return Err(Error::Syntax(SyntaxError {
                                loc: Some(chunks[start_idx].loc.clone()),
                                desc: "expected a number".into(),
                            }));
                        };

                        for chunk in num_chunks.iter() {
                            token.push(chunk.clone());
                        }

                        tokens.push(token);
                    }

                    idx += 1;
                }
                '\'' => {
                    let mut token = Token::new_char_literal();

                    if idx + 2 >= len {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[idx].loc.clone()),
                            desc: "expected a char".into(),
                        }));
                    }

                    idx += 1;

                    chunk = chunks[idx].clone();

                    if chunk.content == '\\'
                        && idx + 1 < len
                        && (chunks[idx + 1].content == '\'' || chunks[idx + 1].content == '\\')
                    {
                        idx += 1;
                        chunk = chunks[idx].clone();
                    }

                    token.push(chunk);

                    if chunks[idx + 1].content == '\'' {
                        tokens.push(token);

                        idx += 2;
                    } else {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[idx].loc.clone()),
                            desc: "expected a char".into(),
                        }));
                    }
                }
                '\"' => {
                    let mut token = Token::new_string_literal();

                    let mut is_string = false;

                    let start_idx = idx;

                    idx += 1;

                    while idx < len && !is_string {
                        chunk = chunks[idx].clone();
                        c = chunk.content;

                        match c {
                            '\\' => {
                                if idx + 1 < len {
                                    idx += 1;
                                    c = chunks[idx].content;
                                    if c == '\\' || c == '\"' {
                                        chunk = chunks[idx].clone();
                                        token.push(chunk.clone());
                                        idx += 1;
                                    }
                                } else {
                                    return Err(Error::Syntax(SyntaxError {
                                        loc: Some(chunks[start_idx].loc.clone()),
                                        desc: "expected a string".into(),
                                    }));
                                }
                            }
                            '\"' => {
                                is_string = true;
                                break;
                            }
                            _ => {
                                token.push(chunk.clone());
                                idx += 1;
                            }
                        }
                    }

                    if is_string {
                        tokens.push(token);

                        idx += 1;
                    } else {
                        return Err(Error::Syntax(SyntaxError {
                            loc: Some(chunks[start_idx].loc.clone()),
                            desc: "expected a string".into(),
                        }));
                    }
                }
                'A'..='z' => {
                    let mut token = Token::new_symbol();
                    token.push(chunk.clone());

                    idx += 1;

                    while idx < len {
                        c = chunks[idx].content;

                        if c.is_ascii_alphanumeric()
                            || (c != COMMENT_MARK
                                && c != FORM_START
                                && c != FORM_END
                                && !c.is_whitespace())
                        {
                            token.push(chunks[idx].clone());

                            idx += 1;
                        } else {
                            break;
                        }
                    }

                    tokens.push(token.clone());
                }
                FORM_START => {
                    forms_count += 1;
                    open_form_idxs.push(idx);
                    let mut is_empty = false;

                    if idx + 1 < len && chunks[idx + 1].content == ')' {
                        is_empty = true;
                    }

                    let mut token = if is_empty {
                        Token::new_empty_literal()
                    } else {
                        Token::new_form_start()
                    };

                    token.push(chunk.clone());

                    if is_empty {
                        idx += 1;

                        forms_count -= 1;
                        open_form_idxs.pop();
                        close_form_idxs.push(idx);

                        token.push(chunks[idx].clone());
                    }

                    tokens.push(token);

                    idx += 1;
                }
                FORM_END => {
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
                _ => idx += 1,
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

            let err_form_chunk = chunks[err_idx].clone();

            return Err(Error::Syntax(SyntaxError {
                loc: Some(err_form_chunk.loc),
                desc,
            }));
        }

        Ok(tokens)
    }

    pub fn from_string(s: String) -> Result<Self> {
        Self::from_str(&s)
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

        let s = "xFFF";

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

        let s = "'\\\\'";

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

        let s = "\"\\\\\""; // "\"

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn symbol_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "aB0c-d.,'ef_!+/9";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Symbol);
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
        assert_eq!(tokens[2].kind, TokenKind::Symbol);
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

        assert!(res.is_ok());

        let tokens = res.unwrap();

        assert_eq!(tokens.len(), 17);
        assert_eq!(tokens[0].kind, TokenKind::DocComment);
        assert_eq!(tokens[1].kind, TokenKind::FormStart);
        assert_eq!(tokens[2].kind, TokenKind::Keyword);
        assert_eq!(tokens[3].kind, TokenKind::Symbol);
        assert_eq!(tokens[13].kind, TokenKind::CharLiteral);
        assert_eq!(tokens[13].chunks.as_ref().unwrap()[0].content, '\'');
        assert_eq!(tokens[14].kind, TokenKind::StringLiteral);
    }
}
