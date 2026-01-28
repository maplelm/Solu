#![allow(unused)]
use super::*;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    pub cursor: usize,
    pub line: usize,
    pub line_start: usize,
    pub src: &'a str,
    pub paren_depth: usize,   // ()
    pub bracket_depth: usize, // []
    pub brace_depth: usize,   // {}
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            tokens: vec![],
            src,
            cursor: 0,
            line_start: 0,
            line: 1,
            paren_depth: 0,
            bracket_depth: 0,
            brace_depth: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexerError> {
        loop {
            self.skip_whitespace();
            match self.peek() {
                '_' | 'a'..='z' | 'A'..='Z' => self.lex_indent(),
                '0'..='9' => {}
                c if matches!(c, '(' | ')' | '[' | ']' | '{' | '}') => self.lex_groupings(c)?,
                c if matches!(c, ':' | ',' | ';' | '\n') => self.lex_delim(c)?,
                c if matches!(
                    c,
                    '.' | '&'
                        | '-'
                        | '>'
                        | '<'
                        | '+'
                        | '*'
                        | '='
                        | '/'
                        | '|'
                        | '^'
                        | '~'
                        | '%'
                        | '!'
                ) =>
                {
                    self.lex_op(c)?
                }
                '\'' => {
                    self.advance();
                    self.lex_char()?;
                }
                '\0' => {
                    self.tokens.push(Token::Special(Special::Eof));
                    return Ok(self.tokens.clone());
                }
                c => {
                    // self.tokens
                    //     .push(Token::Special(Special::Invalid(c.to_string())));
                    // self.advance()
                    return Err(LexerError::new(self, "Invalid token"));
                }
            }
        }
    }

    fn lex_indent(&mut self) {
        let mut indent = String::from(self.peek());
        self.advance();
        while self.is_alphanumeric() || self.peek() == '_' {
            indent.push(self.peek());
            self.advance();
        }
        match indent.as_str() {
            "if" => self.tokens.push(Token::Keyword(Keyword::If)),
            "then" => self.tokens.push(Token::Keyword(Keyword::Then)),
            "else" => self.tokens.push(Token::Keyword(Keyword::Else)),
            "elif" => self.tokens.push(Token::Keyword(Keyword::Elif)),
            "while" => self.tokens.push(Token::Keyword(Keyword::While)),
            "for" => self.tokens.push(Token::Keyword(Keyword::For)),
            "in" => self.tokens.push(Token::Keyword(Keyword::In)),
            "do" => self.tokens.push(Token::Keyword(Keyword::Do)),
            "with" => self.tokens.push(Token::Keyword(Keyword::With)),
            "is" => self.tokens.push(Token::Keyword(Keyword::Is)),
            "end" => self.tokens.push(Token::Keyword(Keyword::End)),
            "return" => self.tokens.push(Token::Keyword(Keyword::Return)),
            "break" => self.tokens.push(Token::Keyword(Keyword::Break)),
            "continue" => self.tokens.push(Token::Keyword(Keyword::Continue)),
            "switch" => self.tokens.push(Token::Keyword(Keyword::Switch)),
            "mut" => self.tokens.push(Token::Keyword(Keyword::Mut)),
            "struct" => self.tokens.push(Token::Keyword(Keyword::Struct)),
            "enum" => self.tokens.push(Token::Keyword(Keyword::Enum)),
            "const" => self.tokens.push(Token::Keyword(Keyword::Const)),
            "type" => self.tokens.push(Token::Keyword(Keyword::Type)),
            "arena" => self.tokens.push(Token::Keyword(Keyword::Arena)),
            "defer" => self.tokens.push(Token::Keyword(Keyword::Defer)),
            "new" => self.tokens.push(Token::Keyword(Keyword::New)),

            "true" => self.tokens.push(Token::Keyword(Keyword::True)),
            "false" => self.tokens.push(Token::Keyword(Keyword::False)),
            "nil" => self.tokens.push(Token::Keyword(Keyword::Nil)),
            "i8" => self.tokens.push(Token::Keyword(Keyword::TypeI8)),
            "i16" => self.tokens.push(Token::Keyword(Keyword::TypeI16)),
            "i32" => self.tokens.push(Token::Keyword(Keyword::TypeI32)),
            "i64" => self.tokens.push(Token::Keyword(Keyword::TypeI64)),
            "u8" => self.tokens.push(Token::Keyword(Keyword::TypeU8)),
            "u16" => self.tokens.push(Token::Keyword(Keyword::TypeU16)),
            "u32" => self.tokens.push(Token::Keyword(Keyword::TypeU32)),
            "u64" => self.tokens.push(Token::Keyword(Keyword::TypeU64)),
            "f32" => self.tokens.push(Token::Keyword(Keyword::TypeF32)),
            "f64" => self.tokens.push(Token::Keyword(Keyword::TypeF64)),
            "char" => self.tokens.push(Token::Keyword(Keyword::TypeChar)),
            "bool" => self.tokens.push(Token::Keyword(Keyword::TypeBool)),

            _ => self.tokens.push(Token::Identifier(indent)),
        }
    }
    fn lex_op(&mut self, c: char) -> Result<(), LexerError> {
        match c {
            '?' => {
                self.tokens.push(Token::Operator(Operator::Terinary));
                self.advance();
            }
            '+' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::PlusEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Plus)),
                }
            }
            '-' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::SubEq));
                        self.advance();
                    }
                    '>' => {
                        self.tokens.push(Token::Operator(Operator::Arrow));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Minus)),
                }
            }
            '*' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::StarEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Star)),
                }
            }
            '/' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::DivEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Div)),
                }
            }
            '=' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::EqEq));
                        self.advance();
                    }
                    '>' => {
                        self.tokens.push(Token::Operator(Operator::FatArrow));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Eq)),
                }
            }
            '>' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::LtEq));
                        self.advance();
                    }
                    '>' => {
                        self.tokens.push(Token::Operator(Operator::ShiftR));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Gt)),
                }
            }
            '<' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::LtEq));
                        self.advance();
                    }
                    '<' => {
                        self.tokens.push(Token::Operator(Operator::ShiftL));
                        self.advance();
                    }
                    '-' => {
                        self.tokens.push(Token::Operator(Operator::ArrowRev));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Lt)),
                }
            }
            '.' => {
                self.advance();
                match self.peek() {
                    '.' => {
                        self.tokens.push(Token::Operator(Operator::Range));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::MemberAccessor)),
                }
            }
            '|' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::PipeEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Pipe)),
                }
            }
            '^' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::CaretEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Caret)),
                }
            }
            '~' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::TildeEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Tilde)),
                }
            }
            '!' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::NotEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Not)),
                }
            }
            '%' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.tokens.push(Token::Operator(Operator::ModEq));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Mod)),
                }
            }
            '&' => {
                self.advance();
                match self.peek() {
                    '&' => {
                        self.tokens.push(Token::Operator(Operator::LAnd));
                        self.advance();
                    }
                    _ => self.tokens.push(Token::Operator(Operator::Amp)),
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn lex_delim(&mut self, c: char) -> Result<(), LexerError> {
        match c {
            ':' => self.tokens.push(Token::Delim(Delimeter::Colon)),
            ',' => self.tokens.push(Token::Delim(Delimeter::Comma)),
            ';' => self.tokens.push(Token::Delim(Delimeter::Term)),
            '\n' => {
                self.advance();
                self.skip_whitespace();
                if !self.in_braces()
                    && self.tokens.len() > 0
                    && self.tokens.last().unwrap_or(&Token::Special(Special::Eof))
                        != &Token::Delim(Delimeter::Term)
                    && self.peek() != '.'
                {
                    self.tokens.push(Token::Delim(Delimeter::Term));
                }
            }
            _ => unreachable!(),
        }
        if c != '\n' {
            self.advance();
        }
        Ok(())
    }

    fn lex_groupings(&mut self, c: char) -> Result<(), LexerError> {
        match c {
            '(' => {
                self.paren_depth = self.paren_depth.wrapping_add(1);
                self.tokens.push(Token::Delim(Delimeter::Lparen));
            }
            ')' => {
                if self.paren_depth <= 0 {
                    return Err(LexerError::new(self, "Unexpected Rparan"));
                }
                self.paren_depth = self.paren_depth.wrapping_sub(1);
                self.tokens.push(Token::Delim(Delimeter::Rparen));
            }
            '[' => {
                self.bracket_depth = self.bracket_depth.wrapping_add(1);
                self.tokens.push(Token::Delim(Delimeter::Lbracket));
            }
            ']' => {
                if self.bracket_depth <= 0 {
                    return Err(LexerError::new(self, "Unexpected Rbracket"));
                }
                self.bracket_depth = self.paren_depth.wrapping_sub(1);
                self.tokens.push(Token::Delim(Delimeter::Rbracket));
            }
            '{' => {
                self.brace_depth = self.brace_depth.wrapping_add(1);
                self.tokens.push(Token::Delim(Delimeter::Lbrace));
            }
            '}' => {
                if self.brace_depth <= 0 {
                    return Err(LexerError::new(self, "Unexpected Rbrace"));
                }
                self.brace_depth = self.brace_depth.wrapping_sub(1);
                self.tokens.push(Token::Delim(Delimeter::Rbrace));
            }
            _ => unreachable!(),
        }
        self.advance();
        Ok(())
    }

    pub fn float_check(&mut self, t: &mut NumberType) -> Result<bool, LexerError> {
        if self.peek() == '.' {
            match t {
                NumberType::Int => *t = NumberType::Float,
                _ => {
                    return Err(LexerError::new(Span::new(self), "Malformed Float"));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn hex_check(&mut self, t: &mut NumberType, ctr: usize) -> Result<bool, LexerError> {
        if self.peek() == 'x' {
            match t {
                NumberType::Int if ctr == 1 => *t = NumberType::Hex,
                _ => {
                    return Err(LexerError::new(
                        Span::new(self),
                        format!("Malformed Hex: {} ({})", self.peek(), ctr),
                    ));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn bin_check(&mut self, t: &mut NumberType, ctr: usize) -> Result<bool, LexerError> {
        if self.peek() == 'b' {
            match t {
                NumberType::Int if ctr == 1 => *t = NumberType::Binary,
                _ => {
                    return Err(LexerError::new(Span::new(self), "Malformed Binary"));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn number_type_check(
        &mut self,
        t: &mut NumberType,
        ctr: usize,
    ) -> Result<bool, LexerError> {
        Ok(self.float_check(t)? || self.hex_check(t, ctr)? || self.bin_check(t, ctr)?)
    }

    pub fn parse_number(&mut self) -> Result<Token, LexerError> {
        let mut n_type = NumberType::Int;
        let mut ctr = 1;
        let mut t = String::from(self.peek());
        self.advance();
        let mut c = self.peek();
        while matches!(
            c,
            '.' | 'x' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F'
        ) || self.is_digit()
        {
            if !self.number_type_check(&mut n_type, ctr)? {
                t.push(c);
            }
            ctr += 1;
            self.advance();
            c = self.peek();
        }
        match n_type {
            NumberType::Int => Ok(Token::Int(t.parse::<u64>().unwrap())),
            NumberType::Float => Ok(Token::Float(t.parse::<f64>().unwrap())),
            NumberType::Hex => Ok(Token::Int(match u64::from_str_radix(&t[1..], 16) {
                Ok(n) => n,
                Err(_) => {
                    return Err(LexerError::new(
                        Span::new(self),
                        format!("Malformed Hex Value: {}", &t[2..]),
                    ));
                }
            })),
            NumberType::Binary => Ok(Token::Int(match u64::from_str_radix(&t[1..], 2) {
                Ok(n) => n,
                Err(_) => {
                    return Err(LexerError::new(Span::new(self), "Malformed Binary Value"));
                }
            })),
        }
    }

    pub fn peek_offset(&self, offset: isize) -> char {
        self.src
            .chars()
            .nth((self.cursor as isize + offset) as usize)
            .unwrap()
    }

    pub fn prev(&self) -> char {
        if self.cursor > self.src.len() || self.cursor == 0 {
            '\0' // end or start of file
        } else {
            self.src.chars().nth(self.cursor - 1).unwrap()
        }
    }

    pub fn peek(&self) -> char {
        if self.cursor >= self.src.len() {
            '\0' // End of file
        } else {
            self.src.chars().nth(self.cursor).unwrap()
        }
    }

    pub fn next(&self) -> char {
        if (self.cursor + 1) >= self.src.len() {
            '\0'
        } else {
            self.src.chars().nth(self.cursor + 1).unwrap()
        }
    }

    pub fn advance(&mut self) {
        let c = self.peek();
        self.cursor += 1;
        if c == '\n' {
            self.line += 1;
            self.line_start = self.cursor;
        }
    }

    pub fn skip_whitespace(&mut self) {
        match self.peek() {
            '\r' | ' ' | '\t' => {
                self.advance();
                self.skip_whitespace();
            }
            '-' if self.peek_offset(1) == '-' => {
                while self.peek() != '\n' {
                    self.advance();
                }
            }
            _ => {}
        }
    }

    pub fn is_digit(&self) -> bool {
        let c = self.peek();
        c >= '0' && c <= '9'
    }

    pub fn is_alpha(&self) -> bool {
        let c = self.peek();
        (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
    }

    pub fn is_alphanumeric(&self) -> bool {
        self.is_alpha() || self.is_digit()
    }

    /*
     *   char_literal ::= "'" (unicode_char | escape_sequence) "'"
     *   escape_sequence ::= "\\" ("'" | "\\" | "n" | "t" | "r" | "u" hex4 | "U" hex8)
     *   unicode_char ::= any single valid Unicode codepoint except '\' or '''
     * */
    pub fn lex_char(&mut self) -> Result<(), LexerError> {
        match self.peek() {
            '\\' => {
                self.advance();
                self.lex_escape_code()?;
                if !self.match_char('\'') {
                    return Err(LexerError::new(Span::new(self), "Improper Character found"));
                }
            }
            c if c != '\'' && c != '\n' => {
                self.tokens.push(Token::Char(c));
                self.advance();
                if !self.match_char('\'') {
                    return Err(LexerError::new(Span::new(self), "Improper Character found"));
                }
            }
            _ => {
                return Err(LexerError::new(Span::new(self), "Improper Character found"));
            }
        }
        Ok(())
    }

    pub fn lex_escape_code(&mut self) -> Result<(), LexerError> {
        match self.peek() {
            'n' => {
                self.advance();
                self.tokens.push(Token::Char('\n'));
                return Ok(());
            }
            't' => {
                self.advance();
                self.tokens.push(Token::Char('\t'));
                return Ok(());
            }
            'r' => {
                self.advance();
                self.tokens.push(Token::Char('\r'));
                return Ok(());
            }
            '\\' => {
                self.advance();
                self.tokens.push(Token::Char('\\'));
                return Ok(());
            }
            '\'' => {
                self.advance();
                self.tokens.push(Token::Char('\''));
                return Ok(());
            }
            'u' => self.lex_unicode_escape(4),
            'U' => self.lex_unicode_escape(8),
            c => Err(LexerError::new(
                self,
                format!("Invalid Escape Sequance: \\{}", c),
            )),
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        let c = self.peek();
        if c == expected {
            self.advance();
        }
        return c == expected;
    }

    fn expected_char(&mut self, expected: char) -> Result<(), LexerError> {
        let c = self.peek();
        if c == expected {
            self.advance();
            return Ok(());
        } else {
            return Err(LexerError::new(
                Span::new(self),
                format!("Unexpected token ({})", c),
            ));
        }
    }

    pub fn lex_unicode_escape(&mut self, size: usize) -> Result<(), LexerError> {
        self.advance();
        let mut value: u32 = 0;
        for _ in 0..size {
            let c = self.peek();
            let num = match c {
                '0'..='9' => c as u32 - '0' as u32,
                'a'..='f' => c as u32 - 'a' as u32 + 10,
                'A'..='F' => c as u32 - 'A' as u32 + 10,
                _ => {
                    return Err(LexerError::new(Span::new(self), "Invalid unicode value"));
                }
            };
            value = (value << 4) | num;
            self.advance();
        }
        if let Some(c) = char::from_u32(value) {
            self.tokens.push(Token::Char(c));
            return Ok(());
        } else {
            return Err(LexerError::new(
                Span::new(self),
                format!("Invalid Char Value: {}", value),
            ));
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                '"' => {
                    let mut str = String::new();
                    self.advance();
                    while self.peek() != '"' {
                        match self.peek() {
                            '\\' => {
                                self.advance();
                                match self.peek() {
                                    '"' => {
                                        str.push('"');
                                        self.advance();
                                    }
                                    'n' => {
                                        str.push('\n');
                                        self.advance();
                                    }
                                    't' => {
                                        str.push('\t');
                                        self.advance();
                                    }
                                    'r' => {
                                        str.push('\r');
                                        self.advance();
                                    }
                                    '\\' => {
                                        str.push('\\');
                                        self.advance();
                                    }
                                    _ => str.push('\\'),
                                }
                            }
                            c => {
                                str.push(c);
                                self.advance();
                            }
                        }
                    }
                    self.advance();
                    tokens.push(Token::String(str));
                }
                _ if self.is_alpha() || self.peek() == '_' => tokens.push(self.parse_indent()),
                _ if self.is_digit() => match self.parse_number() {
                    Ok(t) => tokens.push(t),
                    Err(e) => return Err(e),
                },
            }
        }
        Ok(tokens)
    }

    fn in_braces(&self) -> bool {
        self.paren_depth > 0 || self.bracket_depth > 0 || self.brace_depth > 0
    }
}

enum NumLiteralType {
    Int,
    Float,
    Hex,
    Binary,
}

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}
