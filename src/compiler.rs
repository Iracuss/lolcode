use std::collections::HashMap;

pub trait Compiler {
    // Start getting a token and parse
    fn compile(&mut self, source: &str);
    // Get the next token from the lexer
    fn next_token(&mut self) -> String;
    // Run the syntax analyzer
    fn parse(&mut self);
    // Get the current token
    fn current_token(&self) -> String;
    // Set the current token
    fn set_current_token(&mut self, tok: String);
}

pub trait LexicalAnalyzer {
    // Return the next character from the file/source
    fn get_char(&mut self) -> char;
    // Add a character to the current token
    fn add_char(&mut self, c: char);
    // Lookup a potential token to see if valid
    fn lookup(&self, s: &str) -> bool;
}

pub trait SyntaxAnalyzer {
    // Each parses the given token
    fn parse_lolcode(&mut self) -> Result<(), String>;
    fn parse_head(&mut self) -> Result<(), String>;
    fn parse_title(&mut self) -> Result<(), String>;
    fn parse_comment(&mut self) -> Result<(), String>;
    fn parse_body(&mut self) -> Result<(), String>;
    fn parse_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_text(&mut self) -> Result<(), String>;
    fn parse_variable_define(&mut self) -> Result<(), String>;
    fn parse_variable_use(&mut self) -> Result<(), String>;
    fn parse_bold(&mut self) -> Result<(), String>;
    fn parse_italics(&mut self) -> Result<(), String>;
    fn parse_list(&mut self) -> Result<(), String>;
    fn parse_list_items(&mut self) -> Result<(), String>;
    fn parse_inner_list(&mut self) -> Result<(), String>;
    fn parse_link(&mut self) -> Result<(), String>;
    fn parse_newline(&mut self) -> Result<(), String>;
    fn parse_text(&mut self) -> Result<(), String>;
}

pub trait SemanticAnalyzer {
    // Defines the scope
    fn new_scope(&mut self);
    // Takes off a scope
    fn remove_scope(&mut self) -> String;
    // Adds tokens to the current scope
    fn add_str(&mut self, content: &str);
    // Defines the variable in a hashmap
    fn define_variable(&mut self, name: String, value: String);
    // Looks through a hashmap to use variable
    fn use_variable(&mut self, name: &str) -> String;

}

pub struct LolCompiler {
    pub current_token: String,
    pub source_chars: Vec<char>,
    pub current_index: usize,
    pub current_lexeme: String,
    pub output_stack: Vec<String>,
    pub var_stack: Vec<HashMap<String, String>>
}

impl LolCompiler {
    pub fn new() -> Self {
        LolCompiler {
            current_token: String::new(),
            source_chars: Vec::new(),
            current_index: 0,
            current_lexeme: String::new(),
            output_stack: Vec::new(),
            var_stack: Vec::new()
        }
    }

    pub fn match_token(&mut self, expected: &str) -> Result<(), String> {
        if self.current_token().to_uppercase() == expected.to_uppercase() {
            self.next_token();
            Ok(())
        } else {
            Err(format!("Expected '{}', but found '{}'", expected, self.current_token()))
        }
    }
}