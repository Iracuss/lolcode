use std::{collections::HashMap, process};
use crate::compiler::{LolCompiler, SemanticAnalyzer};

impl SemanticAnalyzer for LolCompiler {
    fn new_scope(&mut self) {
        // Create a new "empty level"
        self.output_stack.push(String::new());
        self.var_stack.push(HashMap::new());
    }

    fn remove_scope(&mut self) -> String {
        self.var_stack.pop();
        let output = self.output_stack.pop().unwrap_or_default();
        output.trim().to_string()
        // Return the whole scope currently
    }

    fn add_str(&mut self, content: &str) {
        // Gets the top of the stack and adds the string to the current string at the top
        let top_index = self.output_stack.len() - 1;
        self.output_stack[top_index].push_str(content);
    }

    fn define_variable(&mut self, name: String, value: String) {
        // Just add to the stack
        let top_index = self.var_stack.len() - 1;
        self.var_stack[top_index].insert(name, value);
    }

    fn use_variable(&mut self, name: &str) -> String {
        // Go through hashmap from top to bottom and get recent var by name
        let mut i = self.var_stack.len();
        while i > 0 {
            i -= 1;
            if self.var_stack[i].contains_key(name) {
                return self.var_stack[i][name].clone();
            }
        }
        // if not then error
        eprintln!("SEMANTIC ERROR: undefined variable: {}", name);
        process::exit(1);
    }
}