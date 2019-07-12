pub struct Parser {
    tokens: Vec<String>
}

impl Parser {
    pub fn new() -> Parser {
        Parser{tokens: Vec::new()}
    }

    pub fn parse(&mut self, src: &String) {
        for letter in src.chars() {
            self.tokens.push(letter.to_string());
        }
        println!("{:?}", &self.tokens);
    }
}
