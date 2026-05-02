use crate::compiler::{LolCompiler, Compiler, LexicalAnalyzer, SyntaxAnalyzer};
use std::process;

impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) {
        self.source_chars = source.chars().collect();
        self.current_index = 0;

        self.next_token();

        self.parse();
    }

    fn next_token(&mut self) -> String {
        self.current_lexeme.clear();
        let mut c = self.get_char();

        // Put a token in
        while c.is_whitespace() && c != '\0' {
            c = self.get_char();
        }

        // Check if we are at the end of file just in case
        if c == '\0' {
            let eof = String::from("EOF");
            self.set_current_token(eof.clone());
            return eof
        }

        // Get all the characters of the token if there is no whitespace or eof
        while !c.is_whitespace() && c != '\0' {
            self.add_char(c);
            c = self.get_char();
        }

        let token = self.current_lexeme.clone();

        // Look up the token if it has #
        if token.starts_with("#") {
            if !self.lookup(&token) {
                eprintln!("Lexical Error: '{}' is not a valid lolcode.", token);
                std::process::exit(1);
            }
        }

        self.set_current_token(token.clone());
        token
    }

    fn parse(&mut self) {
        if let Err(e) = self.parse_lolcode() {
            eprintln!("Syntax Error: {}", e);
            process::exit(1);
        }
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}

impl LexicalAnalyzer for LolCompiler {
    fn get_char(&mut self) -> char {
        // Read from file then return the character
        if self.current_index < self.source_chars.len() {
            let c = self.source_chars[self.current_index];
            self.current_index += 1;
            c
        } else {
            '\0'
        }
    }

    // Just push the character to the end
    fn add_char(&mut self, c: char) {
        self.current_lexeme.push(c);
    }

    // See if we have something correct
    // I think this is all the keywords
    fn lookup(&self, s: &str) -> bool {
        match s.to_uppercase().as_str() {
            "#HAI" | "#KBYE" | "#OBTW" | "#TLDR" | "#MAEK" | "HEAD" | 
            "#MKAY" | "#GIMMEH" | "TITLE" | "#OIC" | "PARAGRAF" | 
            "BOLD" | "ITALICS" | "LIST" | "ITEM" | "NEWLINE" | 
            "LINX" | "#IHAZ" | "#ITIZ" | "#LEMMESEE" => true,
            _ => false,
        }
    }
}
