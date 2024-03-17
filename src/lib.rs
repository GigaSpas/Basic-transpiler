use regex::Regex;

///////////////
///TOKENISER///
///////////////

#[derive(Debug)]
pub enum Bracket{
    Curly,
    Square,
    Paren,
}
#[derive(Debug)]
pub enum Symbol{
    Equals,
    Plus,
    Minus,
    Multiply,
    Devide,
    Modulo,
    Grater,
    Lesser,
    Dot,
    SemiColon,
    Exclamation,
}
#[derive(Debug)]
pub enum Token{
    BracketClose(Bracket),
    BracketOpen(Bracket),
    Number(String),
    String(String),
    Char(String),
    Symbol(Symbol),
}

pub fn tokenise(input: String) -> Result<Vec<Token>, String>{
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut curr_char = chars[i];

        if is_whitespace(&curr_char.to_string()){
            i += 1;
            continue;
        }

        //Deal with multy chacracter strings/number/calls
        if is_letter(&curr_char.to_string()) {
            let mut char = curr_char.to_string();
            i += 1;
            curr_char = chars[i];

            while is_letter(&curr_char.to_string()) || is_number(&curr_char.to_string()) {
            char.push(curr_char);
            i += 1;
            curr_char = chars[i];
            }

            tokens.push(Token::Char(char));
            continue;
        }

        if is_number(&curr_char.to_string()) {
            let mut num = curr_char.to_string();
            i += 1;
            curr_char = chars[i];

            while is_number(&curr_char.to_string()) {
            num.push(curr_char);
            i += 1;
            curr_char = chars[i];
            }

            tokens.push(Token::Number(num));
            continue;
        }

        if curr_char == '\"' {
            let mut str = String::new(); 
            i += 1;
            curr_char = chars[i];

            while curr_char != '\"' {
            //Deal with escaped chars
            if curr_char == '\\'{
            i += 1;
            curr_char = chars[i];
            str.push(curr_char);
            continue;
            }

            str.push(curr_char);
            i += 1;
            curr_char = chars[i];
            }
            
            i += 1;
            tokens.push(Token::String(str));
            continue;
        }

        //Deal with single characters
        match curr_char {

            '=' => {tokens.push(Token::Symbol(Symbol::Equals)); i += 1; continue},
            '-' => {tokens.push(Token::Symbol(Symbol::Minus)); i += 1; continue},
            '+' => {tokens.push(Token::Symbol(Symbol::Plus)); i += 1; continue},
            '*' => {tokens.push(Token::Symbol(Symbol::Multiply)); i += 1; continue},
            '/' => {tokens.push(Token::Symbol(Symbol::Devide)); i += 1; continue},
            '%' => {tokens.push(Token::Symbol(Symbol::Modulo)); i += 1; continue},
            '>' => {tokens.push(Token::Symbol(Symbol::Grater)); i += 1; continue},
            '<' => {tokens.push(Token::Symbol(Symbol::Lesser)); i += 1; continue},
            '.' => {tokens.push(Token::Symbol(Symbol::Dot)); i += 1; continue},
            ';' => {tokens.push(Token::Symbol(Symbol::SemiColon)); i += 1; continue},
            '!' => {tokens.push(Token::Symbol(Symbol::Exclamation)); i += 1; continue},
            '{' => {tokens.push(Token::BracketOpen(Bracket::Curly)); i += 1; continue},
            '}' => {tokens.push(Token::BracketClose(Bracket::Curly)); i += 1; continue},
            '(' => {tokens.push(Token::BracketOpen(Bracket::Paren)); i += 1; continue},
            ')' => {tokens.push(Token::BracketClose(Bracket::Paren)); i += 1; continue},
            '[' => {tokens.push(Token::BracketOpen(Bracket::Square)); i += 1; continue},
            ']' => {tokens.push(Token::BracketClose(Bracket::Square)); i += 1; continue},


            _ => return Err(format!("Tokeniser: Invalid character: {}", curr_char)),
        }
    }

    Ok(tokens)
}

fn is_number(str: &String) -> bool {
    let numbers: Regex = Regex::new(r"[0-9]").unwrap();

    numbers.is_match(str)
}

fn is_letter(str: &String) -> bool {
    let letters: Regex = Regex::new(r"[a-zA-Z]").unwrap();

    letters.is_match(str)
}

fn is_whitespace(str: &String) -> bool {
    let whitespace: Regex = Regex::new(r"\s").unwrap();

    whitespace.is_match(str)
}
