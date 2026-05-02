use crate::compiler::{LolCompiler, SyntaxAnalyzer, Compiler, SemanticAnalyzer};

impl SyntaxAnalyzer for LolCompiler {
    fn parse_lolcode(&mut self) -> Result<(), String> { 
        // Create scope
        // Match for opening and closing tag
        // Every match we add to the add_str the respective tag
        self.new_scope();
        self.match_token("#HAI")?;
        self.add_str("<html>\n");

        // comments (Optional)
        if self.current_token() == "#OBTW" {
            self.parse_comment()?;
        }

        // head (Optional)
        if self.current_token() == "#MAEK" {

            // Make sure next token is not a p tag
            // Go up an index to save the next token then go back one again
            let index = self.current_index;
            self.next_token();
            let next_token = self.current_token().to_uppercase();
            self.current_index = index;
            self.set_current_token(String::from("#MAEK"));

            // Peak ahead
            if next_token == "HEAD" {
                self.parse_head()?;
            }
        }

        // body (Optional but could have nothing but just force a body tag)
        self.add_str("<body>\n");
        self.parse_body()?;

        self.match_token("#KBYE")?;

        // Error if our token is EOF
        if self.current_token() != "EOF" {
            return Err(format!(
                "Expected EOF after #KBYE, but found {}",
                self.current_token()
            ));
        }

        self.add_str("</body>\n</html>\n");

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> { 
        // Match the tokens needed in structure
        // Add to str
        self.match_token("#MAEK")?;
        self.match_token("HEAD")?;
        self.add_str("<head>\n");

        // Can comment
        if self.current_token().to_uppercase() == "#OBTW" {
            self.parse_comment()?;
        }

        // Need a title
        self.parse_title()?;
        self.match_token("#MKAY")?;
        self.add_str("</head>\n");

        Ok(()) 
    }

    fn parse_title(&mut self) -> Result<(), String> { 
        // Match and add to str the proper tags
        self.match_token("#GIMMEH")?;
        self.match_token("TITLE")?;
        self.add_str("<title>");

        // Go through the whole text until #OIC
        while self.current_token().to_uppercase() != "#OIC" {
            // If we see EOF then fail
            if self.current_token() == "EOF" {
                return Err(String::from("Unexpected EOF inside title, expected #OIC"));
            }
            // Get the text and add it to the stack with a space then go next
            let text = self.current_token();
            self.add_str(&format!("{} ", text));
            self.next_token();
        }

        // Match and add
        self.match_token("#OIC")?;
        self.add_str("</title>\n");
   
        Ok(()) 
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        // Match and add
        self.match_token("#OBTW")?;
        self.add_str("<!-- ");

        // Go through the whole text until #TLDR
        while self.current_token().to_uppercase() != "#TLDR" {
            if self.current_token() == "EOF" {
                return Err(String::from("Unexpected EOF inside comment, expected #TLDR"));
            }
            // Get the text and add it to the stack with a space then go next
            let text = self.current_token();
            self.add_str(&format!("{} ", text));
            self.next_token();
        }

        // Match and add
        self.match_token("#TLDR")?;
        self.add_str("-->\n");

        Ok(()) 
    }

    // Very loose rules due to how bodies work in HTML
    fn parse_body(&mut self) -> Result<(), String> { 
        // Loop and match the token given until a break happens
        loop {
            match self.current_token().to_uppercase().as_str() {
                // comment
                "#OBTW" => self.parse_comment()?,
                // var define
                "#IHAZ" => self.parse_variable_define()?,
                // use var
                "#LEMMESEE" => self.parse_variable_use()?,
                // bold, italics, newline, and link since they all use gimmeh
                "#GIMMEH" => {
                    self.match_token("#GIMMEH")?;
                    match self.current_token().to_uppercase().as_str() {
                        "BOLD" => self.parse_bold()?,
                        "ITALICS" => self.parse_italics()?,
                        "LINX" => self.parse_link()?,
                        "NEWLINE" => self.parse_newline()?,
                        other => return Err(format!(
                            "Unexpected token after #GIMMEH in body: '{}'", 
                            other
                        )),
                    }
                }
                // p tag and lists
                "#MAEK" => {
                    self.match_token("#MAEK")?;
                    match self.current_token().to_uppercase().as_str() {
                        "PARAGRAF" => self.parse_paragraph()?,
                        "LIST" => self.parse_list()?,
                        other => return Err(format!(
                            "Unexpected token after #MAEK in body: '{}'", other
                        )),
                    }
                }
                // end tag
                "#KBYE" | "#MKAY" | "EOF" => break,
                // Plain text
                _ => self.parse_text()?,
            }
        }
        Ok(())
    }

    fn parse_paragraph(&mut self) -> Result<(), String> { 
        // Match then create a new scope
        self.match_token("PARAGRAF")?;
        self.new_scope();

        // Comment
        if self.current_token().to_uppercase() == "#OBTW" {
            self.parse_comment()?;
        }

        // Go into the inner paragraph and then match
        self.parse_inner_paragraph()?;
        self.match_token("#MKAY")?;

        // Add the text to str by popping off the scope
        let inner = self.remove_scope();
        self.add_str(&format!("<p>{}</p>\n", inner));

        Ok(()) 
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        // Inner text which is just a special body function
        self.parse_inner_text()
    }

    fn parse_inner_text(&mut self) -> Result<(), String> { 
        // Loop and match the token given until a break happens
        loop {
            match self.current_token().to_uppercase().as_str() {
                // var define
                "#IHAZ" => self.parse_variable_define()?,
                // comment
                "#OBTW" => self.parse_comment()?,
                // use var
                "#LEMMESEE" => self.parse_variable_use()?,
                // bold, italics, newline, and link since they all use gimmeh
                "#GIMMEH" => {
                    self.match_token("#GIMMEH")?;
                    match self.current_token().to_uppercase().as_str() {
                        "BOLD" => self.parse_bold()?,
                        "ITALICS" => self.parse_italics()?,
                        "LINX" => self.parse_link()?,
                        "NEWLINE" => self.parse_newline()?,
                        other => return Err(format!(
                            "Unexpected token after #GIMMEH in inner_text: '{}'", 
                            other
                        )),
                    }
                }
                // Lists
                "#MAEK" => {
                    self.match_token("#MAEK")?;
                    match self.current_token().to_uppercase().as_str() {
                        "LIST" => self.parse_list()?,
                        other => return Err(format!(
                            "Unexpected token after #MAEK in inner_text: '{}'", other
                        )),
                    }
                }
                // end tag
                "#KBYE" | "#MKAY" | "EOF" => break,
                // Plain text
                _ => self.parse_text()?,
            }
        }
        Ok(()) 
    }
    fn parse_variable_define(&mut self) -> Result<(), String> {
        // Match all the correct tokens
        self.match_token("#IHAZ")?;

        // take in token for name
        let name = self.current_token();
        self.next_token();

        self.match_token("#ITIZ")?;

        // take in token for value
        let value = self.current_token();
        self.next_token();

        // Define the variable/put into the stack
        self.match_token("#MKAY")?;
        self.define_variable(name, value);

        Ok(())
    }
    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.match_token("#LEMMESEE")?;

        // use token to make a name
        let name = self.current_token();
        self.next_token();

        self.match_token("#OIC")?;

        // Look to see if token was a var name then return it and add to str
        let value = self.use_variable(&name);
        self.add_str(&format!("{} ", value));

        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> { 
        // Same match and add pattern
        self.match_token("BOLD")?;
        self.add_str("<b>");

        // Loop through and match to see if you are text or a var or need to end
        loop {
            match self.current_token().to_uppercase().as_str() {
                "#LEMMESEE" => self.parse_variable_use()?,
                "#OIC" => break,
                "EOF" => return Err(String::from("Unexpected EOF inside bold")), // error check
                _ => self.parse_text()?,
            }
        }
        
        // Same pattern
        self.match_token("#OIC")?;
        self.add_str("</b> ");
        Ok(()) 
    }
    fn parse_italics(&mut self) -> Result<(), String> { 
        // Match and add pattern
        self.match_token("ITALICS")?;
        self.add_str("<i>");
        
        // While it doesn't have #OIC loop through
        while self.current_token().to_uppercase() != "#OIC" {
            // Error check
            if self.current_token().to_uppercase() == "EOF" {
                return Err(String::from("Unexpected EOF inside italics, expected #OIC"));
            }

            // add to str stack and go next token
            let text = self.current_token();
            self.add_str(&format!("{} ", text));
            self.next_token();
        }
        
        self.match_token("#OIC")?;
        self.add_str("</i> ");
        Ok(()) 
    }

    fn parse_list(&mut self) -> Result<(), String> {
        // match then make a new scope,
        self.match_token("LIST")?;
        self.new_scope();

        // List items
        self.parse_list_items()?;
        self.match_token("#MKAY")?;

        // pop the whole scope content into the text and add to str
        let text = self.remove_scope();
        self.add_str(&format!("<ul>{}</ul>\n", text));

        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        // If we are not at the end or EOF loop
        while self.current_token().to_uppercase() != "#MKAY" {
            if self.current_token() == "EOF" {
                return Err(String::from("Unexpected EOF inside list_items, expected #OIC"));
            }

            // Match tokens then go into inner list
            self.match_token("#GIMMEH")?;
            self.match_token("ITEM")?;
            self.parse_inner_list()?;
        }

        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        // Need to create scope, well not really but not a bad idea
        self.new_scope();
        loop {
            match self.current_token().to_uppercase().as_str() {
                // use var
                "#LEMMESEE" => self.parse_variable_use()?,
                // bold, italics, and link since they all use gimmeh
                "#GIMMEH" => {
                    self.match_token("#GIMMEH")?;
                    match self.current_token().to_uppercase().as_str() {
                        "BOLD" => self.parse_bold()?,
                        "ITALICS" => self.parse_italics()?,
                        "LINX" => self.parse_link()?,
                        other => return Err(format!(
                            "Unexpected token after #GIMMEH in inner_list: '{}'", 
                            other
                        )),
                    }
                }
                // end tag
                "#KBYE" | "#OIC" | "#MKAY" | "EOF" => break,
                // Plain text
                _ => self.parse_text()?,
            }
        }
        // Match then add to str
        self.match_token("#OIC")?;
        let text = self.remove_scope();
        self.add_str(&format!("<li>{}</li>\n", text));

        Ok(())
    }

    fn parse_link(&mut self) -> Result<(), String> { 
        // Match then check if EOF
        self.match_token("LINX")?;

        if self.current_token() == "EOF" {
            return Err(String::from("Unexpected EOF, expected link"));
        }

        // Text then match and add token into the str stack
        let text = self.current_token();
        self.next_token();
        self.match_token("#OIC")?;
        self.add_str(&format!("<a href=\"{}\">{}</a>", text, text));

        Ok(()) 
    }

    fn parse_newline(&mut self) -> Result<(), String> { 
        // Match and add to str
        self.match_token("NEWLINE")?;
        self.add_str("<br>\n");
        Ok(()) 
    }

    fn parse_text(&mut self) -> Result<(), String> { 
        // Check for EOF
        if self.current_token() == "EOF" {
            return Err(String::from("Unexpected EOF, expected text"));
        }
        // Add text to str stack then go to next token
        let text = self.current_token();
        self.add_str(&format!("{} ", text));
        self.next_token();
        Ok(()) 
    }
}