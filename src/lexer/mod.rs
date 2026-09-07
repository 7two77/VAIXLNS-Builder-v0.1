#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(u64),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Arrow,
    Dot,
    KeywordLink,
    KeywordRequires,
    KeywordProvides,
    KeywordDomain,
    KeywordKind,
    KeywordMeta,
    KeywordOntology,
    KeywordLaws,
    KeywordSovereignty,
    KeywordConsistency,
    KeywordLineage,
    EOF,
    Unknown(char),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return self.token(TokenKind::EOF);
        }
        let ch = self.input[self.pos];
        match ch {
            '{' => { self.advance(); self.token(TokenKind::LBrace) }
            '}' => { self.advance(); self.token(TokenKind::RBrace) }
            '[' => { self.advance(); self.token(TokenKind::LBracket) }
            ']' => { self.advance(); self.token(TokenKind::RBracket) }
            ':' => { self.advance(); self.token(TokenKind::Colon) }
            ',' => { self.advance(); self.token(TokenKind::Comma) }
            '.' => { self.advance(); self.token(TokenKind::Dot) }
            '-' if self.peek() == Some('>') => {
                self.pos += 2; self.column += 2;
                self.token(TokenKind::Arrow)
            }
            '"' => self.lex_string(),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(),
            '0'..='9' => self.lex_number(),
            _ => { self.advance(); self.token(TokenKind::Unknown(ch)) }
        }
    }

    fn lex_string(&mut self) -> Token {
        self.advance();
        let start_line = self.line;
        let start_col = self.column;
        let mut value = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            if self.input[self.pos] == '\\' {
                self.advance();
                if self.pos < self.input.len() {
                    match self.input[self.pos] {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => value.push(self.input[self.pos]),
                    }
                    self.advance();
                }
            } else {
                value.push(self.input[self.pos]);
                self.advance();
            }
        }
        if self.pos < self.input.len() {
            self.advance();
        }
        Token {
            kind: TokenKind::StringLiteral(value),
            line: start_line,
            column: start_col,
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut value = String::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let kind = match value.as_str() {
            "link" => TokenKind::KeywordLink,
            "requires" => TokenKind::KeywordRequires,
            "provides" => TokenKind::KeywordProvides,
            "domain" => TokenKind::KeywordDomain,
            "kind" => TokenKind::KeywordKind,
            "meta" => TokenKind::KeywordMeta,
            "ontology" => TokenKind::KeywordOntology,
            "laws" => TokenKind::KeywordLaws,
            "sovereignty" => TokenKind::KeywordSovereignty,
            "consistency" => TokenKind::KeywordConsistency,
            "lineage" => TokenKind::KeywordLineage,
            _ => TokenKind::Identifier(value),
        };
        Token { kind, line: start_line, column: start_col }
    }

    fn lex_number(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut value = String::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Token {
            kind: TokenKind::NumberLiteral(value.parse().unwrap_or(0)),
            line: start_line,
            column: start_col,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                ' ' | '\t' => { self.advance(); }
                '\n' => { self.line += 1; self.column = 1; self.pos += 1; }
                '\r' => { self.pos += 1; }
                _ => break,
            }
        }
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
            self.column += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token { kind, line: self.line, column: self.column }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if let TokenKind::EOF = token.kind {
            break;
        }
    }
    tokens
}
