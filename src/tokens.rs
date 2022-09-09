use crate::chunks::Chunks;
use crate::error::{Error, SyntaxError};
use crate::keyword::KEYWORDS;
use crate::keyword::{COMMENT_MARK, COMMENT_MARK_POSTFIX};
use crate::keyword::{FORM_END, FORM_START};
use crate::result::Result;
use crate::token::Token;
use std::fs;
use std::ops;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Default)]
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

                        if keyword != &&chars {
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
                '0'..='9' | 'b' | 'o' | 'x' => {
                    let mut token = Token::new_uint_literal();
                    token.push(chunk.clone());

                    let radix = match c {
                        'b' => 2,
                        'o' => 8,
                        'x' => 16,
                        _ => 10,
                    };

                    let mut uint_idx = idx;
                    uint_idx += 1;

                    while uint_idx < len {
                        let uint_chunk = chunks[uint_idx].clone();

                        if uint_chunk.content.is_digit(radix) {
                            token.push(uint_chunk);
                            uint_idx += 1;
                        } else {
                            break;
                        }
                    }

                    if uint_idx > idx + 1 || c.is_digit(radix) {
                        tokens.push(token);
                        idx = uint_idx;
                    } else {
                        let mut token = Token::new_symbol();
                        token.push(chunk.clone());
                        tokens.push(token);
                        idx += 1;
                    }
                }
                '-' | '+' => {
                    let mut num_chunks = Vec::new();
                    num_chunks.push(chunk.clone());

                    let mut num_idx = idx + 1;

                    chunk = chunks[num_idx].clone();
                    c = chunk.content;

                    match c {
                        'b' | 'o' | 'x' => {
                            let radix = match c {
                                'b' => 2,
                                'o' => 8,
                                'x' => 16,
                                _ => unreachable!(),
                            };

                            chunk = chunks[num_idx].clone();
                            num_chunks.push(chunk);

                            num_idx += 1;

                            while num_idx < len {
                                chunk = chunks[num_idx].clone();

                                if chunk.content.is_digit(radix) {
                                    num_chunks.push(chunk.clone());
                                    num_idx += 1;
                                } else {
                                    break;
                                }
                            }

                            if num_idx > idx + 2 {
                                let mut token = Token::new_int_literal();

                                for chunk in num_chunks {
                                    token.push(chunk);
                                }

                                tokens.push(token);
                                idx = num_idx;
                            } else {
                                idx += 1;
                            }
                        }
                        '0'..='9' => {
                            let mut has_point = false;
                            let mut has_exp = false;

                            chunk = chunks[num_idx].clone();
                            num_chunks.push(chunk);

                            num_idx += 1;
                            let radix = 10;

                            while num_idx < len {
                                chunk = chunks[num_idx].clone();
                                c = chunk.content;

                                if c.is_digit(radix) {
                                    num_chunks.push(chunk.clone());
                                    num_idx += 1;
                                } else if c == '.' && !has_point {
                                    if num_idx + 1 >= len {
                                        break;
                                    }

                                    if chunks[num_idx + 1].content.is_digit(radix) {
                                        has_point = true;
                                        num_chunks.push(chunk.clone());
                                        num_idx += 1;
                                    } else {
                                        break;
                                    }
                                } else if c == 'E' && has_point && !has_exp {
                                    if num_idx + 2 >= len {
                                        break;
                                    }

                                    let sign_chunk = chunks[num_idx + 1].clone();
                                    let exp_chunk = chunks[num_idx + 2].clone();

                                    if sign_chunk.content != '-' && sign_chunk.content != '+' {
                                        break;
                                    }

                                    if !exp_chunk.content.is_digit(radix) {
                                        break;
                                    }

                                    has_exp = true;
                                    num_chunks.push(chunk.clone());
                                    num_chunks.push(sign_chunk);
                                    num_idx += 2;
                                } else {
                                    break;
                                }
                            }

                            if num_idx > idx + 1 {
                                let mut token = if has_point {
                                    Token::new_float_literal()
                                } else {
                                    Token::new_int_literal()
                                };

                                for chunk in num_chunks {
                                    token.push(chunk);
                                }

                                tokens.push(token);
                                idx = num_idx;
                            } else {
                                idx += 1;
                            }
                        }
                        _ => idx += 1,
                    }
                }
                '\'' => {
                    let mut token = Token::new_char_literal();

                    if idx + 2 >= len {
                        return Err(Error::Syntax(SyntaxError {
                            loc: chunks[idx].loc.clone(),
                            desc: "expected a char".into(),
                        }));
                    }

                    idx += 1;

                    chunk = chunks[idx].clone();

                    if chunk.content == '\\' {
                        if idx + 2 < len {
                            if chunks[idx + 2].content == '\'' {
                                idx += 1;
                            }
                        }
                    }

                    token.push(chunk);

                    if chunks[idx + 1].content == '\'' {
                        tokens.push(token);

                        idx += 2;
                    } else {
                        return Err(Error::Syntax(SyntaxError {
                            loc: chunks[idx].loc.clone(),
                            desc: "expected a char".into(),
                        }));
                    }
                }
                '\"' => {
                    let mut token = Token::new_string_literal();

                    let mut is_string = false;
                    let mut escape_idx = 0;

                    let prev_idx = idx;

                    idx += 1;

                    while idx < len && !is_string {
                        chunk = chunks[idx].clone();

                        if chunk.content == '\\' {
                            if escape_idx == idx - 1 {
                                escape_idx = 0;
                            } else {
                                escape_idx = idx;
                            }
                        }

                        if chunk.content == '"' && !(escape_idx == idx - 1) {
                            is_string = true;
                            break;
                        }

                        idx += 1;

                        token.push(chunk.clone());
                    }

                    if is_string {
                        tokens.push(token);

                        idx += 1;
                    } else {
                        return Err(Error::Syntax(SyntaxError {
                            loc: chunks[prev_idx].loc.clone(),
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

                    let mut token = Token::new_form_start();
                    token.push(chunk.clone());

                    tokens.push(token);

                    idx += 1;
                }
                FORM_END => {
                    forms_count -= 1;

                    if forms_count < 0 {
                        return Err(Error::Syntax(SyntaxError {
                            loc: chunks[idx].loc.clone(),
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
                loc: err_form_chunk.loc,
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

        let s = "'\"' '\'' 'a'";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::CharLiteral);
        assert_eq!(tokens[1].kind, TokenKind::CharLiteral);
        assert_eq!(tokens[2].kind, TokenKind::CharLiteral);
    }

    #[test]
    fn string_literal_tokens() {
        use super::Tokens;
        use crate::token::TokenKind;

        let s = "\"this is a str\\\"ing\" \"string\"";

        let tokens = Tokens::from_str(s).unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[1].kind, TokenKind::StringLiteral);
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
                assert_eq!(loc.pos, 2);

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
                assert_eq!(loc.pos, 0);

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
        assert_eq!(tokens[14].kind, TokenKind::StringLiteral);
    }
}
