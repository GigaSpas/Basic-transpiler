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

pub fn tokenise(input: String) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut curr_char = chars[i];

        if is_whitespace(&curr_char.to_string()){
            continue;
        }

        if is_letter(&curr_char.to_string()) {
            let mut char = curr_char.to_string();
            i += 1;
            curr_char = chars[i];

            while is_letter(&curr_char.to_string()) {
            char.push(curr_char);
            i += 1;
            curr_char = chars[i];
            println!("1 {i}");
            }
            i += 1;
            tokens.push(Token::Char(char));
            continue;

        }

        if is_number(&curr_char.to_string()) {
            let mut num= curr_char.to_string();
            i += 1;
            curr_char = chars[i];

            while is_number(&curr_char.to_string()) {
            num.push(curr_char);
            i += 1;
            curr_char = chars[i];
            println!("1 {i}");
            }
            i += 1;
            tokens.push(Token::Number(num));
            continue;

        }


        println!("{i}");
        i += 1;
        continue;
    }

    tokens
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
