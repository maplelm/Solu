use std::collections::HashMap;
use std::fmt;

pub enum NumberType {
    Int,
    Float,
    Hex,
    Binary,
}

pub struct LexerError {
    pub span: Span,
    pub msg: String,
}

impl LexerError {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        Self {
            span,
            msg: msg.into(),
        }
    }
}

// Used to track where lexer is in file for error messaging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize, // Byte offset
    pub end: usize,   // Byte offset Exclusive
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(lex: &Lexer) -> Self {
        Self {
            start: lex.cursor,
            end: lex.cursor,
            line: lex.line,
            col: lex.cursor - lex.line_start,
        }
    }

    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    // Literals
    Ident(String),
    Int(u64),
    Float(f64),
    String(String),
    Char(char),

    //type keywords
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeF32,
    TypeF64,
    TypeChar,
    TypeBool,

    // Control Flow
    If,
    Then,
    Else,
    Elif,
    While,
    For,
    In,
    Do,
    With,
    Is,
    End,
    Return,
    Break,
    Continue,
    Match,

    // Declarations
    Mut,
    Struct,
    Enum,
    Const,
    Type,

    // Memory
    Arena,
    // Sizeof,
    // Typeof,
    Defer,
    New,
    Ref,
    Ptr,

    // Literals (Values)
    True,  // true
    False, // false
    Nil,   // nil

    // Logic Operators
    And, // and
    Or,  // or
    Not, // !

    // Comparisons
    EqEq,  // ==
    NotEq, // !=
    Gt,    // >
    Lt,    // <
    GtEq,  // >=
    LtEq,  // <=

    // Assignment
    Eq,     // =
    PlusEq, // +=
    SubEq,  // -=
    StarEq, // *=
    DivEq,  // /=

    // Arithmatic
    Plus,  // +
    Star,  // *
    Div,   // /
    Minus, // -
    Mod,   // %

    // Bitwise
    Amp,   // &
    Pipe,  // |
    Caret, // ^
    Tilde, // ~
    LtLt,  // <<
    GtGt,  // >>

    // Delimeters
    Lparen,   // (
    Rparen,   // )
    Lbracket, // [
    Rbracket, // ]
    Lbrace,   // {
    Rbrace,   // }
    Dot,      // .
    DotStar,  // .* (Dereference)
    Arrow,    // ->
    ArrowRev, // <-
    FatArrow, // =>
    //DotDot,   // ..
    //DotDotEq, // ..=
    Question, // ?
    Semi,     // ;
    Comma,    // ,
    Colon,    // :

    // Special
    Term, // statement termination
    Eof,
    Invalid(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "IDE({})", s),
            Token::Int(n) => write!(f, "INT({})", n),
            Token::Float(fl) => write!(f, "FLOAT({})", fl),
            Token::String(s) => write!(f, "STR({})", s),
            Token::Char(c) => write!(f, "C({})", c),
            Token::TypeI8 => write!(f, "I8"),
            Token::TypeI16 => write!(f, "I16"),
            Token::TypeI32 => write!(f, "I32"),
            Token::TypeI64 => write!(f, "I64"),
            Token::TypeU8 => write!(f, "U8"),
            Token::TypeU16 => write!(f, "U16"),
            Token::TypeU32 => write!(f, "U32"),
            Token::TypeU64 => write!(f, "U64"),
            Token::TypeF32 => write!(f, "F32"),
            Token::TypeF64 => write!(f, "F64"),
            Token::TypeChar => write!(f, "CHAR"),
            Token::TypeBool => write!(f, "BOOL"),
            Token::If => write!(f, "IF"),
            Token::Then => write!(f, "THEN"),
            Token::Else => write!(f, "ELS"),
            Token::Elif => write!(f, "ELIF"),
            Token::While => write!(f, "WHL"),
            Token::For => write!(f, "FOR"),
            Token::In => write!(f, "IN"),
            Token::Do => write!(f, "DO"),
            Token::End => write!(f, "END"),
            Token::Return => write!(f, "RET"),
            Token::Break => write!(f, "BRK"),
            Token::Continue => write!(f, "CON"),
            Token::Match => write!(f, "MAT"),
            // Token::Let => write!(f, "let"),
            Token::Mut => write!(f, "MUT"),
            // Token::Fn => write!(f, "fn"),
            Token::Struct => write!(f, "STRU"),
            Token::Enum => write!(f, "ENUM"),
            // Token::Trait => write!(f, "trait"),
            // Token::Impl => write!(f, "impl"),
            Token::Const => write!(f, "CONST"),
            Token::Type => write!(f, "TYPE"),
            // Token::Pub => write!(f, "pub"),
            // Token::Import => write!(f, "import"),
            Token::Arena => write!(f, "ARENA"),
            // Token::Sizeof => write!(f, "sizeof"),
            // Token::Typeof => write!(f, "typeof"),
            Token::Defer => write!(f, "DEFER"),
            Token::New => write!(f, "NEW"),
            Token::Ref => write!(f, "REF"),
            Token::Ptr => write!(f, "PTR"),
            Token::True => write!(f, "TRUE"),
            Token::False => write!(f, "FALSE"),
            Token::Nil => write!(f, "NIL"),
            Token::And => write!(f, "AND"),
            Token::Or => write!(f, "OR"),
            Token::Not => write!(f, "NOT"),
            Token::EqEq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Gt => write!(f, ">"),
            Token::Lt => write!(f, "<"),
            Token::GtEq => write!(f, ">="),
            Token::LtEq => write!(f, "<="),
            Token::Eq => write!(f, "="),
            Token::PlusEq => write!(f, "+="),
            Token::SubEq => write!(f, "-="),
            Token::StarEq => write!(f, "*="),
            Token::DivEq => write!(f, "/="),
            Token::Plus => write!(f, "+"),
            Token::Star => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Minus => write!(f, "-"),
            Token::Mod => write!(f, "%"),
            Token::Amp => write!(f, "&"),
            Token::Pipe => write!(f, "|"),
            Token::Caret => write!(f, "^"),
            Token::Tilde => write!(f, "~"),
            Token::LtLt => write!(f, "<<"),
            Token::GtGt => write!(f, ">>"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbracket => write!(f, "["),
            Token::Rbracket => write!(f, "]"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Dot => write!(f, "."),
            Token::DotStar => write!(f, ".*"),
            Token::Arrow => write!(f, "->"),
            Token::ArrowRev => write!(f, "<-"),
            Token::FatArrow => write!(f, "=>"),
            // Token::DotDot => write!(f, ".."),
            // Token::DotDotEq => write!(f, "..="),
            Token::Question => write!(f, "?"),
            Token::Semi => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Term => write!(f, "TERM"),
            Token::Eof => write!(f, "EOF"),
            Token::Invalid(s) => write!(f, "INVALID({})", s),
            Token::With => write!(f, "WITH"),
            Token::Is => write!(f, "IS"),
        }
    }
}

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub keywords: HashMap<&'static str, Token>,
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
        let keywords = HashMap::from([
            ("do", Token::Do),
            ("with", Token::With),
            ("is", Token::Is),
            ("end", Token::End),
            ("if", Token::If),
            ("then", Token::Then),
            ("else", Token::Else),
            ("elif", Token::Elif),
            ("while", Token::While),
            ("for", Token::For),
            ("in", Token::In),
            ("return", Token::Return),
            ("break", Token::Break),
            ("continue", Token::Continue),
            // ("let", Token::Let),
            ("mut", Token::Mut),
            // ("fn", Token::Fn),
            // ("trait", Token::Trait),
            // ("impl", Token::Impl),
            ("true", Token::True),
            ("false", Token::False),
            ("nil", Token::Nil),
            ("and", Token::And),
            ("or", Token::Or),
            ("not", Token::Not),
            ("arena", Token::Arena),
            ("defer", Token::Defer),
            // ("sizeof", Token::Sizeof),
            // ("typeof", Token::Typeof),
            ("i8", Token::TypeI8),
            ("i16", Token::TypeI16),
            ("i32", Token::TypeI32),
            ("i64", Token::TypeI64),
            ("u8", Token::TypeU8),
            ("u16", Token::TypeU16),
            ("u32", Token::TypeU32),
            ("u64", Token::TypeU64),
            ("f32", Token::TypeF32),
            ("f64", Token::TypeF64),
            ("char", Token::TypeChar),
            ("bool", Token::TypeBool),
            ("true", Token::True),
            ("false", Token::False),
        ]);
        Self {
            keywords,
            src,
            cursor: 0,
            line_start: 0,
            line: 1,
            paren_depth: 0,
            bracket_depth: 0,
            brace_depth: 0,
        }
    }

    pub fn parse_indent(&mut self) -> Token {
        let mut t = String::from(self.peek());
        self.advance();
        while self.is_alphanumeric() || self.peek() == '_' {
            t.push(self.peek());
            self.advance();
        }

        if self.keywords.contains_key(t.as_str()) {
            self.keywords.get(t.as_str()).unwrap().clone()
        } else {
            Token::Ident(t)
        }
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
        if self.cursor + 1 < self.src.len() {
            let c = self.peek();
            match c {
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
    pub fn lex_char(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        match self.peek() {
            '\\' => {
                self.advance();
                self.lex_escape_code(tokens)?;
                if !self.match_char('\'') {
                    return Err(LexerError::new(Span::new(self), "Improper Character found"));
                }
            }
            c if c != '\'' && c != '\n' => {
                tokens.push(Token::Char(c));
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

    pub fn lex_escape_code(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexerError> {
        match self.peek() {
            'n' => {
                self.advance();
                tokens.push(Token::Char('\n'));
                return Ok(());
            }
            't' => {
                self.advance();
                tokens.push(Token::Char('\t'));
                return Ok(());
            }
            'r' => {
                self.advance();
                tokens.push(Token::Char('\r'));
                return Ok(());
            }
            '\\' => {
                self.advance();
                tokens.push(Token::Char('\\'));
                return Ok(());
            }
            '\'' => {
                self.advance();
                tokens.push(Token::Char('\''));
                return Ok(());
            }
            'u' => self.lex_unicode_escape(4, tokens),
            'U' => self.lex_unicode_escape(8, tokens),
            c => Err(LexerError::new(
                Span::new(self),
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

    pub fn lex_unicode_escape(
        &mut self,
        size: usize,
        tokens: &mut Vec<Token>,
    ) -> Result<(), LexerError> {
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
            tokens.push(Token::Char(c));
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
                '(' => {
                    self.paren_depth = self.paren_depth.wrapping_add(1);
                    tokens.push(Token::Lparen);
                    self.advance();
                }
                ')' => {
                    if self.paren_depth <= 0 {
                        return Err(LexerError::new(Span::new(self), "Unexpected Rparan"));
                    }
                    self.paren_depth = self.paren_depth.wrapping_sub(1);
                    tokens.push(Token::Rparen);
                    self.advance();
                }
                '[' => {
                    self.bracket_depth = self.bracket_depth.wrapping_add(1);
                    tokens.push(Token::Lbracket);
                    self.advance();
                }
                ']' => {
                    if self.bracket_depth <= 0 {
                        return Err(LexerError::new(Span::new(self), "Unexpected Rbracket"));
                    }
                    self.bracket_depth = self.bracket_depth.wrapping_sub(1);
                    tokens.push(Token::Rbracket);
                    self.advance();
                }
                '{' => {
                    self.brace_depth = self.brace_depth.wrapping_add(1);
                    tokens.push(Token::Lbrace);
                    self.advance();
                }
                '}' => {
                    if self.brace_depth <= 0 {
                        return Err(LexerError::new(Span::new(self), "Unexpected Rbrace"));
                    }
                    self.bracket_depth = self.brace_depth.wrapping_sub(1);
                    tokens.push(Token::Rbrace);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.advance();
                }
                '&' => {
                    tokens.push(Token::Amp);
                    self.advance();
                }
                '-' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::SubEq);
                            self.advance();
                        }
                        '>' => {
                            tokens.push(Token::Arrow);
                            self.advance();
                        }
                        _ => tokens.push(Token::Minus),
                    }
                }
                '>' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::GtEq);
                            self.advance();
                        }
                        '>' => {
                            tokens.push(Token::GtGt);
                            self.advance();
                        }
                        _ => tokens.push(Token::Gt),
                    }
                }
                '<' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::LtEq);
                            self.advance();
                        }
                        '-' => {
                            tokens.push(Token::ArrowRev);
                            self.advance();
                        }
                        '<' => {
                            tokens.push(Token::LtLt);
                            self.advance();
                        }
                        _ => tokens.push(Token::Lt),
                    }
                }
                '+' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::PlusEq);
                            self.advance();
                        }
                        _ => tokens.push(Token::Plus),
                    }
                }
                '*' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::StarEq);
                            self.advance();
                        }
                        _ => tokens.push(Token::Star),
                    }
                }
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
                '=' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::EqEq);
                            self.advance();
                        }
                        '>' => {
                            tokens.push(Token::FatArrow);
                            self.advance();
                        }
                        _ => tokens.push(Token::Eq),
                    }
                }
                '/' => {
                    self.advance();
                    match self.peek() {
                        '=' => {
                            tokens.push(Token::DivEq);
                            self.advance();
                        }
                        _ => tokens.push(Token::Div),
                    }
                }
                '\n' => {
                    self.advance();
                    self.skip_whitespace();
                    if !self.in_braces()
                        && tokens.len() > 0
                        && *tokens.last().unwrap_or(&Token::Eof) != Token::Term
                        && self.peek() != '.'
                    {
                        tokens.push(Token::Term);
                    }
                }
                _ if self.is_alpha() || self.peek() == '_' => tokens.push(self.parse_indent()),
                _ if self.is_digit() => match self.parse_number() {
                    Ok(t) => tokens.push(t),
                    Err(e) => return Err(e),
                },
                '\'' => {
                    self.advance();
                    self.lex_char(&mut tokens)?;
                }
                '\0' => {
                    tokens.push(Token::Eof);
                    break;
                }
                c => {
                    tokens.push(Token::Invalid(c.to_string()));
                    self.advance();
                    /*
                    return Err(LexerError::new(
                        Span::new(
                            self.cursor,
                            self.cursor,
                            self.line,
                            self.cursor - self.line_start,
                        ),
                        "Invalid Token",
                    ));
                    */
                }
            }
        }
        Ok(tokens)
    }

    fn in_braces(&self) -> bool {
        self.paren_depth > 0 || self.bracket_depth > 0 || self.brace_depth > 0
    }
}
