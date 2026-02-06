#![allow(unused)]
use super::*;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub span_table: Vec<Span>,
    token_len: usize,
    pub tokens: Vec<Token>,
    pub cursor: usize,
    pub line: usize,
    pub line_start: usize,
    pub src: String,
    pub paren_depth: usize,   // ()
    pub bracket_depth: usize, // []
    pub brace_depth: usize,   // {}
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            token_len: 0,
            tokens: vec![],
            span_table: vec![],
            src,
            cursor: 0,
            line_start: 0,
            line: 1,
            paren_depth: 0,
            bracket_depth: 0,
            brace_depth: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<LexerObject>, LexerError> {
        // Stip ending characters until final end

        while self.src.len() > 0 && self.src.chars().last().unwrap_or('\0') != 'd' {
            self.src.pop();
        }

        loop {
            self.skip_whitespace();
            match self.peek() {
                '_' | 'a'..='z' | 'A'..='Z' => self.lex_indent(),
                '0'..='9' => self.parse_number()?,
                c if matches!(c, '(' | ')' | '[' | ']' | '{' | '}') => self.lex_groupings(c)?,
                c if matches!(c, ':' | ',' | ';' | '\n') => self.lex_delim(c)?,
                c if matches!(c, '"') => self.lex_string_literal(c),
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
                    self.push_token(Token::Eof);
                    return Ok(
                        match LexerObject::from_vectors(&self.tokens, &self.span_table) {
                            Ok(lo) => lo,
                            Err(s) => return Err(LexerError::new(self, s)),
                        },
                    );
                }
                c => return Err(LexerError::new(self, "Invalid token")),
            }
        }
    }

    fn lex_string_literal(&mut self, c: char) {
        self.advance();
        let mut str_literal = String::new();
        while self.peek() != '"' {
            match self.peek() {
                '\\' => {
                    self.advance();
                    match self.peek() {
                        '"' => {
                            str_literal.push('"');
                            self.advance();
                        }
                        'n' => {
                            str_literal.push('\n');
                            self.advance();
                        }
                        't' => {
                            str_literal.push('\t');
                            self.advance();
                        }
                        'r' => {
                            str_literal.push('\r');
                            self.advance();
                        }
                        '\\' => {
                            str_literal.push('\\');
                            self.advance();
                        }
                        _ => str_literal.push('\\'),
                    }
                }
                c => {
                    str_literal.push(c);
                    self.advance();
                }
            }
        }
        self.advance();
        self.push_token(Token::Str(str_literal));
    }

    fn lex_indent(&mut self) {
        let mut indent = String::from(self.peek());
        self.advance();
        while self.is_alphanumeric() || self.peek() == '_' {
            indent.push(self.peek());
            self.advance();
        }
        match indent.as_str() {
            "if" => self.push_token(Token::If),
            "then" => self.push_token(Token::Then),
            "else" => self.push_token(Token::Else),
            "elif" => self.push_token(Token::Elif),
            "while" => self.push_token(Token::While),
            "for" => self.push_token(Token::For),
            "in" => self.push_token(Token::In),
            "do" => self.push_token(Token::Do),
            "with" => self.push_token(Token::With),
            "is" => self.push_token(Token::Is),
            "end" => self.push_token(Token::End),
            "return" => self.push_token(Token::Return),
            "break" => self.push_token(Token::Break),
            "continue" => self.push_token(Token::Continue),
            "switch" => self.push_token(Token::Switch),
            "mut" => self.push_token(Token::Mut),
            "struct" => self.push_token(Token::Struct),
            "enum" => self.push_token(Token::Enum),
            "const" => self.push_token(Token::Const),
            "type" => self.push_token(Token::Type),
            "arena" => self.push_token(Token::Arena),
            "defer" => self.push_token(Token::Defer),
            "new" => self.push_token(Token::New),
            "namespace" => self.push_token(Token::Namespace),

            "true" => self.push_token(Token::True),
            "false" => self.push_token(Token::False),
            "nil" => self.push_token(Token::Nil),
            "i8" => self.push_token(Token::TypeI8),
            "i16" => self.push_token(Token::TypeI16),
            "i32" => self.push_token(Token::TypeI32),
            "i64" => self.push_token(Token::TypeI64),
            "u8" => self.push_token(Token::TypeU8),
            "u16" => self.push_token(Token::TypeU16),
            "u32" => self.push_token(Token::TypeU32),
            "u64" => self.push_token(Token::TypeU64),
            "f32" => self.push_token(Token::TypeF32),
            "f64" => self.push_token(Token::TypeF64),
            "char" => self.push_token(Token::TypeChar),
            "bool" => self.push_token(Token::TypeBool),
            "this" => self.push_token(Token::This),
            "case" => self.push_token(Token::Case),
            "default" => self.push_token(Token::Default),

            _ => self.push_token(Token::Identifier(indent)),
        }
    }
    fn lex_op(&mut self, c: char) -> Result<(), LexerError> {
        match c {
            '?' => {
                self.push_token(Token::Terinary);
                self.advance();
            }
            '+' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::PlusEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Plus),
                }
            }
            '-' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::SubEq);
                        self.advance();
                    }
                    '>' => {
                        self.push_token(Token::Arrow);
                        self.advance();
                    }
                    _ => self.push_token(Token::Minus),
                }
            }
            '*' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::StarEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Star),
                }
            }
            '/' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::DivEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Div),
                }
            }
            '=' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::EqEq);
                        self.advance();
                    }
                    '>' => {
                        self.push_token(Token::FatArrow);
                        self.advance();
                    }
                    _ => self.push_token(Token::Eq),
                }
            }
            '>' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::LtEq);
                        self.advance();
                    }
                    '>' => {
                        self.push_token(Token::ShiftR);
                        self.advance();
                    }
                    _ => self.push_token(Token::Gt),
                }
            }
            '<' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::LtEq);
                        self.advance();
                    }
                    '<' => {
                        self.push_token(Token::ShiftL);
                        self.advance();
                    }
                    '-' => {
                        self.push_token(Token::ArrowRev);
                        self.advance();
                    }
                    _ => self.push_token(Token::Lt),
                }
            }
            '.' => {
                self.advance();
                match self.peek() {
                    '.' => {
                        self.push_token(Token::Range);
                        self.advance();
                    }
                    _ => self.push_token(Token::MemberAccessor),
                }
            }
            '|' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::PipeEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Pipe),
                }
            }
            '^' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::CaretEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Caret),
                }
            }
            '~' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::TildeEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Tilde),
                }
            }
            '!' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::NotEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Not),
                }
            }
            '%' => {
                self.advance();
                match self.peek() {
                    '=' => {
                        self.push_token(Token::ModEq);
                        self.advance();
                    }
                    _ => self.push_token(Token::Mod),
                }
            }
            '&' => {
                self.advance();
                match self.peek() {
                    '&' => {
                        self.push_token(Token::LAnd);
                        self.advance();
                    }
                    _ => self.push_token(Token::Amp),
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn lex_delim(&mut self, c: char) -> Result<(), LexerError> {
        match c {
            ':' => self.push_token(Token::Colon),
            ',' => self.push_token(Token::Comma),
            ';' => self.push_token(Token::Term),
            '\n' => {
                self.advance();
                self.skip_whitespace();
                if !self.in_braces()
                    && self.tokens.len() > 0
                    && self.tokens.last().unwrap_or(&Token::Eof) != &Token::Term
                    && self.peek() != '.'
                {
                    self.push_token(Token::Term);
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
                self.push_token(Token::Lparen);
            }
            ')' => {
                if self.paren_depth <= 0 {
                    return Err(LexerError::new(self, "Unexpected Rparan"));
                }
                self.paren_depth = self.paren_depth.wrapping_sub(1);
                self.push_token(Token::Rparen);
            }
            '[' => {
                self.bracket_depth = self.bracket_depth.wrapping_add(1);
                self.push_token(Token::Lbracket);
            }
            ']' => {
                if self.bracket_depth <= 0 {
                    return Err(LexerError::new(self, "Unexpected Rbracket"));
                }
                self.bracket_depth = self.bracket_depth.wrapping_sub(1);
                self.push_token(Token::Rbracket);
            }
            '{' => {
                self.brace_depth = self.brace_depth.wrapping_add(1);
                self.push_token(Token::Lbrace);
            }
            '}' => {
                if self.brace_depth <= 0 {
                    return Err(LexerError::new(self, "Unexpected Rbrace"));
                }
                self.brace_depth = self.brace_depth.wrapping_sub(1);
                self.push_token(Token::Rbrace);
            }
            _ => unreachable!(),
        }
        self.advance();
        Ok(())
    }

    fn float_check(&mut self, t: &mut NumLiteralType) -> Result<bool, LexerError> {
        if self.peek() == '.' {
            match t {
                NumLiteralType::Int => *t = NumLiteralType::Float,
                _ => {
                    return Err(LexerError::new(self, "Malformed Float"));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn hex_check(&mut self, t: &mut NumLiteralType, ctr: usize) -> Result<bool, LexerError> {
        if self.peek() == 'x' {
            match t {
                NumLiteralType::Int if ctr == 1 => *t = NumLiteralType::Hex,
                _ => {
                    return Err(LexerError::new(
                        self,
                        format!("Malformed Hex: {} ({})", self.peek(), ctr),
                    ));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn bin_check(&mut self, t: &mut NumLiteralType, ctr: usize) -> Result<bool, LexerError> {
        if self.peek() == 'b' {
            match t {
                NumLiteralType::Int if ctr == 1 => *t = NumLiteralType::Binary,
                _ => {
                    return Err(LexerError::new(self, "Malformed Binary"));
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn number_type_check(
        &mut self,
        t: &mut NumLiteralType,
        ctr: usize,
    ) -> Result<bool, LexerError> {
        Ok(self.float_check(t)? || self.hex_check(t, ctr)? || self.bin_check(t, ctr)?)
    }

    fn parse_number(&mut self) -> Result<(), LexerError> {
        let mut type_state: NumLiteralType = NumLiteralType::Int;
        let mut ctr = 1;
        let mut t = String::from(self.peek());
        self.advance();
        let mut c = self.peek();
        while matches!(
            c,
            '.' | 'x' | 'a'..='f' | 'A'..='F'
        ) || self.is_digit()
        {
            self.number_type_check(&mut type_state, ctr)?;
            t.push(c);
            ctr += 1;
            self.advance();
            c = self.peek();
        }
        match type_state {
            NumLiteralType::Int => self.push_token(Token::Int(match t.parse::<u64>() {
                Ok(n) => n,
                Err(e) => {
                    return Err(LexerError::new(
                        self,
                        format!("Failed to parse int literal: {}", e),
                    ));
                }
            })),
            NumLiteralType::Float => self.push_token(Token::Float(match t.parse::<f64>() {
                Ok(f) => f,
                Err(e) => {
                    return Err(LexerError::new(
                        self,
                        format!("Failed to parse float literal: {}", e),
                    ));
                }
            })),
            NumLiteralType::Hex => {
                self.push_token(Token::Int(match u64::from_str_radix(&t[2..], 16) {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(LexerError::new(
                            self,
                            format!("Malformed Hex Value: {}", &t[2..]),
                        ));
                    }
                }))
            }
            NumLiteralType::Binary => {
                self.push_token(Token::Int(match u64::from_str_radix(&t[2..], 2) {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(LexerError::new(self, "Malformed Binary Value"));
                    }
                }))
            }
        };
        Ok(())
    }

    fn peek_offset(&self, offset: isize) -> char {
        self.src
            .chars()
            .nth((self.cursor as isize + offset) as usize)
            .unwrap()
    }

    fn prev(&self) -> char {
        if self.cursor > self.src.len() || self.cursor == 0 {
            '\0' // end or start of file
        } else {
            self.src.chars().nth(self.cursor - 1).unwrap()
        }
    }

    fn peek(&self) -> char {
        if self.cursor >= self.src.len() {
            '\0' // End of file
        } else {
            self.src.chars().nth(self.cursor).unwrap()
        }
    }

    fn next(&self) -> char {
        if (self.cursor + 1) >= self.src.len() {
            '\0'
        } else {
            self.src.chars().nth(self.cursor + 1).unwrap()
        }
    }

    fn advance(&mut self) {
        let c = self.peek();
        self.cursor += 1;
        if c == '\n' {
            self.line += 1;
            self.line_start = self.cursor;
        }
    }

    fn skip_whitespace(&mut self) {
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

    fn is_digit(&self) -> bool {
        let c = self.peek();
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self) -> bool {
        let c = self.peek();
        (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
    }

    fn is_alphanumeric(&self) -> bool {
        self.is_alpha() || self.is_digit()
    }

    fn lex_char(&mut self) -> Result<(), LexerError> {
        match self.peek() {
            '\\' => {
                self.advance();
                self.lex_escape_code()?;
                if !self.match_char('\'') {
                    return Err(LexerError::new(self, "Improper Character found"));
                }
            }
            c if c != '\'' && c != '\n' => {
                self.push_token(Token::Char(c));
                self.advance();
                if !self.match_char('\'') {
                    return Err(LexerError::new(self, "Improper Character found"));
                }
            }
            _ => {
                return Err(LexerError::new(self, "Improper Character found"));
            }
        }
        Ok(())
    }

    pub fn lex_escape_code(&mut self) -> Result<(), LexerError> {
        match self.peek() {
            'n' => {
                self.advance();
                self.push_token(Token::Char('\n'));
                return Ok(());
            }
            't' => {
                self.advance();
                self.push_token(Token::Char('\t'));
                return Ok(());
            }
            'r' => {
                self.advance();
                self.push_token(Token::Char('\r'));
                return Ok(());
            }
            '\\' => {
                self.advance();
                self.push_token(Token::Char('\\'));
                return Ok(());
            }
            '\'' => {
                self.advance();
                self.push_token(Token::Char('\''));
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
            return Err(LexerError::new(self, format!("Unexpected token ({})", c)));
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
                    return Err(LexerError::new(self, "Invalid unicode value"));
                }
            };
            value = (value << 4) | num;
            self.advance();
        }
        if let Some(c) = char::from_u32(value) {
            self.push_token(Token::Char(c));
            return Ok(());
        } else {
            return Err(LexerError::new(
                self,
                format!("Invalid Char Value: {}", value),
            ));
        }
    }

    fn in_braces(&self) -> bool {
        self.paren_depth > 0 || self.bracket_depth > 0 || self.brace_depth > 0
    }

    fn push_token(&mut self, tok: Token) {
        self.span_table.push(Span::new(self));
        self.tokens.push(tok);
        self.token_len += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
