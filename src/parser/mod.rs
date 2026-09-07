// Parser module - VAIXLNS LNS parser
// VAIXLNS Builder v0.1

use crate::lexer::{Token, TokenKind};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Document, String> {
        let mut document = Document::new();
        
        while !self.is_eof() {
            let token = self.peek();
            
            match &token.kind {
                TokenKind::KeywordMeta => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        let meta = self.parse_meta()?;
                        self.expect_rbrace()?;
                        document.decls.push(Decl::Meta(meta));
                    }
                }
                TokenKind::Identifier(id) if id == "vaixlns_root" => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_root(&mut document)?;
                        self.expect_rbrace()?;
                    }
                }
                TokenKind::KeywordOntology => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_ontology(&mut document)?;
                        self.expect_rbrace()?;
                    }
                }
                TokenKind::KeywordLaws => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_laws(&mut document)?;
                        self.expect_rbrace()?;
                    }
                }
                TokenKind::KeywordLink => {
                    self.consume();
                    let from = self.consume_identifier()?;
                    self.expect_arrow()?;
                    let to = self.consume_identifier()?;
                    document.decls.push(Decl::Relation(RelationDecl { from, to }));
                }
                _ => {
                    self.consume();
                }
            }
        }
        
        Ok(document)
    }

    fn parse_root(&mut self, document: &mut Document) -> Result<(), String> {
        while !self.is_rbrace() && !self.is_eof() {
            let token = self.peek();
            
            match &token.kind {
                TokenKind::KeywordMeta => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        let meta = self.parse_meta()?;
                        self.expect_rbrace()?;
                        document.decls.push(Decl::Meta(meta));
                    }
                }
                TokenKind::KeywordOntology => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_ontology(document)?;
                        self.expect_rbrace()?;
                    }
                }
                TokenKind::KeywordLaws => {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_laws(document)?;
                        self.expect_rbrace()?;
                    }
                }
                _ => {
                    self.consume();
                }
            }
        }
        Ok(())
    }

    fn parse_ontology(&mut self, document: &mut Document) -> Result<(), String> {
        while !self.is_rbrace() && !self.is_eof() {
            let token = self.peek();
            
            if let TokenKind::Identifier(id) = &token.kind {
                if id == "domains" {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_domains(document)?;
                        self.expect_rbrace()?;
                    }
                } else if id == "entities" {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_entities(document)?;
                        self.expect_rbrace()?;
                    }
                } else if id == "relations" {
                    self.consume();
                    if self.is_lbrace() {
                        self.consume();
                        self.parse_relations(document)?;
                        self.expect_rbrace()?;
                    }
                } else {
                    self.consume();
                }
            } else {
                self.consume();
            }
        }
        Ok(())
    }

    fn parse_domains(&mut self, document: &mut Document) -> Result<(), String> {
        while !self.is_rbrace() && !self.is_eof() {
            if let TokenKind::Identifier(id) = self.peek().kind {
                let domain_id = id;
                self.consume();
                
                let domain_name = if self.is_string_literal() {
                    if let TokenKind::StringLiteral(s) = self.consume().kind {
                        s
                    } else {
                        String::new()
                    }
                } else if self.is_identifier() {
                    let s = self.consume_identifier()?;
                    s
                } else {
                    String::new()
                };
                
                document.decls.push(Decl::Domain(DomainDecl {
                    id: domain_id,
                    name: domain_name,
                }));
            } else {
                self.consume();
            }
        }
        Ok(())
    }

    fn parse_entities(&mut self, document: &mut Document) -> Result<(), String> {
        while !self.is_rbrace() && !self.is_eof() {
            while !self.is_eof() && !self.is_identifier() && !self.is_rbrace() {
                self.consume();
            }
            
            if self.is_eof() || self.is_rbrace() { break; }
            
            let entity_id = self.consume_identifier()?;
            
            if self.is_lbrace() {
                self.consume();
                let mut entity = EntityDecl {
                    id: entity_id.clone(),
                    domain: String::new(),
                    kind: String::new(),
                    provides: Vec::new(),
                    requires: Vec::new(),
                };
                
                while !self.is_eof() && !self.is_rbrace() {
                    while !self.is_eof() && !self.is_identifier() && !self.is_rbrace() {
                        self.consume();
                    }
                    
                    if self.is_eof() || self.is_rbrace() { break; }
                    
                    let key = self.consume_identifier()?;
                    
                    while !self.is_eof() && !self.is_string_literal() && !self.is_identifier() && !self.is_lbracket() && !self.is_rbrace() {
                        self.consume();
                    }
                    
                    if self.is_eof() || self.is_rbrace() { break; }
                    
                    let value = if self.is_string_literal() {
                        if let TokenKind::StringLiteral(s) = self.consume().kind {
                            s
                        } else {
                            String::new()
                        }
                    } else if self.is_identifier() {
                        self.consume_identifier()?
                    } else if self.is_lbracket() {
                        self.consume();
                        let mut items = Vec::new();
                        while !self.is_eof() && !self.is_rbracket() {
                            if self.is_string_literal() {
                                if let TokenKind::StringLiteral(s) = self.consume().kind {
                                    items.push(s);
                                }
                            } else if self.is_identifier() {
                                let s = self.consume_identifier()?;
                                items.push(s);
                            } else {
                                break;
                            }
                            if self.is_comma() {
                                self.consume();
                            }
                        }
                        self.expect_rbracket()?;
                        format!("[{}]", items.join(","))
                    } else {
                        String::new()
                    };
                    
                    match key.as_str() {
                        "domain" => entity.domain = value,
                        "kind" => entity.kind = value,
                        "provides" => entity.provides = parse_list(&value)?,
                        "requires" => entity.requires = parse_list(&value)?,
                        _ => {}
                    }
                }
                self.expect_rbrace()?;
                document.decls.push(Decl::Entity(entity));
            }
        }
        Ok(())
    }

    fn parse_relations(&mut self, document: &mut Document) -> Result<(), String> {
        while !self.is_rbrace() && !self.is_eof() {
            if self.is_keyword_link() {
                self.consume();
                let from = self.consume_identifier()?;
                self.expect_arrow()?;
                let to = self.consume_identifier()?;
                document.decls.push(Decl::Relation(RelationDecl { from, to }));
            } else {
                self.consume();
            }
        }
        Ok(())
    }

    fn parse_laws(&mut self, document: &mut Document) -> Result<(), String> {
        while !self.is_rbrace() && !self.is_eof() {
            let token = self.peek();
            
            if let TokenKind::Identifier(id) = &token.kind {
                let category = id.clone();
                self.consume();
                
                if self.is_lbrace() {
                    self.consume();
                    let mut rules = Vec::new();
                    
                    while !self.is_eof() && !self.is_rbrace() {
                        if let TokenKind::Identifier(rule_id) = self.peek().kind {
                            self.consume();
                            
                            if self.is_string_literal() {
                                if let TokenKind::StringLiteral(desc) = self.consume().kind {
                                    rules.push(LawRule {
                                        id: rule_id,
                                        description: desc,
                                        enforce: None,
                                    });
                                }
                            } else {
                                self.consume();
                            }
                        } else {
                            self.consume();
                        }
                    }
                    self.expect_rbrace()?;
                    
                    document.decls.push(Decl::Law(LawDecl {
                        category,
                        rules,
                    }));
                }
            } else {
                self.consume();
            }
        }
        Ok(())
    }

    fn parse_meta(&mut self) -> Result<MetaDecl, String> {
        let mut meta = MetaDecl {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            authority: String::new(),
        };
        
        while !self.is_rbrace() && !self.is_eof() {
            let key = self.consume_identifier()?;
            
            if self.is_string_literal() {
                if let TokenKind::StringLiteral(s) = self.consume().kind {
                    match key.as_str() {
                        "id" => meta.id = s,
                        "name" => meta.name = s,
                        "version" => meta.version = s,
                        "authority" => meta.authority = s,
                        _ => return Err(format!("Unknown meta attribute: {}", key)),
                    }
                }
            } else if self.is_identifier() {
                let s = self.consume_identifier()?;
                match key.as_str() {
                    "id" => meta.id = s,
                    "name" => meta.name = s,
                    "version" => meta.version = s,
                    "authority" => meta.authority = s,
                    _ => return Err(format!("Unknown meta attribute: {}", key)),
                }
            } else {
                self.consume();
            }
        }
        
        Ok(meta)
    }

    fn is_lbrace(&self) -> bool { matches!(self.peek().kind, TokenKind::LBrace) }
    fn is_rbrace(&self) -> bool { matches!(self.peek().kind, TokenKind::RBrace) }
    fn is_lbracket(&self) -> bool { matches!(self.peek().kind, TokenKind::LBracket) }
    fn is_rbracket(&self) -> bool { matches!(self.peek().kind, TokenKind::RBracket) }
    fn is_comma(&self) -> bool { matches!(self.peek().kind, TokenKind::Comma) }
    fn is_arrow(&self) -> bool { matches!(self.peek().kind, TokenKind::Arrow) }
    fn is_string_literal(&self) -> bool { matches!(self.peek().kind, TokenKind::StringLiteral(_)) }
    fn is_identifier(&self) -> bool { matches!(self.peek().kind, TokenKind::Identifier(_)) }
    fn is_keyword_link(&self) -> bool { matches!(self.peek().kind, TokenKind::KeywordLink) }

    fn expect_rbrace(&mut self) -> Result<(), String> {
        if self.is_rbrace() {
            self.consume();
            Ok(())
        } else {
            Err(format!("Expected '}}', got {:?}", self.peek().kind))
        }
    }

    fn expect_rbracket(&mut self) -> Result<(), String> {
        if self.is_rbracket() { 
            self.consume(); 
            Ok(()) 
        } else { 
            Err("Expected ']'".to_string()) 
        }
    }

    fn expect_arrow(&mut self) -> Result<(), String> {
        if self.is_arrow() { 
            self.consume(); 
            Ok(()) 
        } else { 
            Err(format!("Expected '->', got {:?}", self.peek().kind)) 
        }
    }

    fn consume_identifier(&mut self) -> Result<String, String> {
        match self.consume().kind {
            TokenKind::Identifier(s) => Ok(s),
            TokenKind::StringLiteral(s) => Ok(s),
            _ => Err("Expected identifier".to_string()),
        }
    }

    fn peek(&self) -> Token {
        if self.pos < self.tokens.len() {
            self.tokens[self.pos].clone()
        } else {
            Token { kind: TokenKind::EOF, line: 0, column: 0 }
        }
    }

    fn consume(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::EOF)
    }
}

fn parse_list(value: &str) -> Result<Vec<String>, String> {
    if !value.starts_with('[') || !value.ends_with(']') {
        return Ok(Vec::new());
    }
    let inner = &value[1..value.len() - 1];
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    Ok(inner.split(',').map(|s| s.trim().to_string()).collect())
}
