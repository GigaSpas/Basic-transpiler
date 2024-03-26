use regex::Regex;
use std::error::Error;
//use std::fs::File;

///////////////
///TOKENISER///
///////////////

#[derive(Debug, Clone)]
pub enum Bracket {
    Curly,
    Square,
    Paren,
}
#[derive(Debug, Clone, Copy)]
pub enum Symbol {
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
#[derive(Debug, Clone)]
pub enum Token {
    BracketClose(Bracket),
    BracketOpen(Bracket),
    Number(String),
    String(String),
    Char(String),
    Symbol(Symbol),
}

pub fn tokenise(input: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut curr_char = chars[i];

        if is_whitespace(&curr_char.to_string()) {
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
                if curr_char == '\\' {
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
            '=' => {
                tokens.push(Token::Symbol(Symbol::Equals));
                i += 1;
                continue;
            }
            '-' => {
                tokens.push(Token::Symbol(Symbol::Minus));
                i += 1;
                continue;
            }
            '+' => {
                tokens.push(Token::Symbol(Symbol::Plus));
                i += 1;
                continue;
            }
            '*' => {
                tokens.push(Token::Symbol(Symbol::Multiply));
                i += 1;
                continue;
            }
            '/' => {
                tokens.push(Token::Symbol(Symbol::Devide));
                i += 1;
                continue;
            }
            '%' => {
                tokens.push(Token::Symbol(Symbol::Modulo));
                i += 1;
                continue;
            }
            '>' => {
                tokens.push(Token::Symbol(Symbol::Grater));
                i += 1;
                continue;
            }
            '<' => {
                tokens.push(Token::Symbol(Symbol::Lesser));
                i += 1;
                continue;
            }
            '.' => {
                tokens.push(Token::Symbol(Symbol::Dot));
                i += 1;
                continue;
            }
            ';' => {
                tokens.push(Token::Symbol(Symbol::SemiColon));
                i += 1;
                continue;
            }
            '!' => {
                tokens.push(Token::Symbol(Symbol::Exclamation));
                i += 1;
                continue;
            }
            '{' => {
                tokens.push(Token::BracketOpen(Bracket::Curly));
                i += 1;
                continue;
            }
            '}' => {
                tokens.push(Token::BracketClose(Bracket::Curly));
                i += 1;
                continue;
            }
            '(' => {
                tokens.push(Token::BracketOpen(Bracket::Paren));
                i += 1;
                continue;
            }
            ')' => {
                tokens.push(Token::BracketClose(Bracket::Paren));
                i += 1;
                continue;
            }
            '[' => {
                tokens.push(Token::BracketOpen(Bracket::Square));
                i += 1;
                continue;
            }
            ']' => {
                tokens.push(Token::BracketClose(Bracket::Square));
                i += 1;
                continue;
            }

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

////////////
///PARSER///
////////////

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    StringLiteral(String),
    NumberLiteral(String),
    Symbol(Symbol),
    Veriable {
        name: String,
        value: String,
    },
    VeriableCall(String),
    IfStatement {
        condition: Vec<Node>,
        body: Vec<Node>,
    },
    ElseStatement {
        body: Vec<Node>,
    },
    ForLoop {
        var_name: String,
        start: String,
        end: String,
        body: Vec<Node>,
    },
    WhileLoop {
        condition: Vec<Node>,
        body: Vec<Node>,
    },
    Function {
        name: String,
        input: Vec<Node>,
        body: Vec<Node>,
    },
    FunctionCall {
        name: String,
        input: Vec<Node>,
    },
}

pub struct Output {
    pub node: Node,
    end_num: usize,
}

pub fn parser(start: usize, tokens: Vec<Token>) -> Result<Output, Box<dyn Error>> {
    let mut ast: Vec<Node> = Vec::new();
    let mut i = start;

    while i < tokens.len() {
        let mut curr_token = &tokens[i];

        if let Token::Char(a) = curr_token {
            if a == "let" {
                let name;
                let value;
                i += 1;
                curr_token = &tokens[i];
                if let Token::Char(b) = curr_token {
                    name = b.to_string();
                } else {
                    return Err(format!("Parser: Expected char got: {:?}", curr_token).into());
                }

                i += 2;
                curr_token = &tokens[i];

                match curr_token {
                    Token::Char(c) => value = c.to_string(),
                    Token::Number(n) => value = n.to_string(),
                    Token::String(s) => value = s.to_string(),
                    _ => {
                        return Err(format!(
                            "Parser: Expected Char/String/Number got: {:?}",
                            curr_token
                        )
                        .into())
                    }
                }
                i += 1;
                ast.push(Node::Veriable { name, value });
                continue;
            }

            if a == "if" {
                // Create the condition
                let mut condition: Vec<Node> = vec![];
                i += 1;
                curr_token = &tokens[i];

                while !matches!(*curr_token, Token::BracketOpen(Bracket::Curly)) {
                    match curr_token {
                        Token::Number(n) => condition.push(Node::NumberLiteral(n.to_string())),
                        Token::String(s) => condition.push(Node::StringLiteral(s.to_string())),
                        Token::Char(c) => condition.push(Node::VeriableCall(c.to_string())),
                        Token::Symbol(s) => condition.push(Node::Symbol(*s)),
                        _ => {
                            return Err(format!(
                                "Parser: Expected Char/String/Number got: {:?}",
                                curr_token
                            )
                            .into())
                        }
                    }
                    i += 1;
                    curr_token = &tokens[i];
                }
                i += 1;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                match result.node {
                    Node::Program(body) => ast.push(Node::IfStatement { condition, body }),
                    _ => return Err("Error".into()),
                }
                i = result.end_num + 1;

                continue;
            }

            if a == "else" {
                i += 2;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                match result.node {
                    Node::Program(body) => ast.push(Node::ElseStatement { body }),
                    _ => return Err("Error".into()),
                }
                i = result.end_num + 1;

                continue;
            }

            if a == "while" {
                // Create the condition
                let mut condition: Vec<Node> = vec![];
                i += 1;
                curr_token = &tokens[i];

                while !matches!(*curr_token, Token::BracketOpen(Bracket::Curly)) {
                    match curr_token {
                        Token::Number(n) => condition.push(Node::NumberLiteral(n.to_string())),
                        Token::String(s) => condition.push(Node::StringLiteral(s.to_string())),
                        Token::Char(c) => condition.push(Node::VeriableCall(c.to_string())),
                        Token::Symbol(s) => condition.push(Node::Symbol(*s)),
                        _ => {
                            return Err(format!(
                                "Parser: Expected Char/String/Number got: {:?}",
                                curr_token
                            )
                            .into())
                        }
                    }
                    i += 1;
                    curr_token = &tokens[i];
                }
                i += 1;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                match result.node {
                    Node::Program(body) => ast.push(Node::WhileLoop { condition, body }),
                    _ => return Err("Error".into()),
                }
                i = result.end_num + 1;

                continue;
            }

            if a == "for" {
                // Create the condition
                i += 1;
                curr_token = &tokens[i];
                let var_name;
                let start;
                if let Token::Char(c) = curr_token {
                    var_name = c.to_string();
                } else {
                    return Err(format!("Parser: Expected Char got: {:?}", curr_token).into());
                }

                i += 2;
                curr_token = &tokens[i];

                match curr_token {
                    Token::Number(n) => start = n.to_string(),
                    Token::Char(c) => start = c.to_string(),
                    _ => {
                        return Err(
                            format!("Parser: Expected Num/Char got: {:?}", curr_token).into()
                        )
                    }
                }

                i += 3;
                curr_token = &tokens[i];
                let end;

                if let Token::Symbol(_) = curr_token {
                    i += 1;
                    curr_token = &tokens[i];

                    match curr_token {
                        Token::Number(n) => end = (n.parse::<usize>()? - 1).to_string(),
                        Token::Char(c) => end = (c.parse::<usize>()? - 1).to_string(),
                        _ => {
                            return Err(
                                format!("Parser: Expected Num/Char got: {:?}", curr_token).into()
                            )
                        }
                    }
                } else if let Token::Number(_) = curr_token {
                    match curr_token {
                        Token::Number(n) => end = n.to_string(),
                        Token::Char(c) => end = c.to_string(),
                        _ => {
                            return Err(
                                format!("Parser: Expected Num/Char got: {:?}", curr_token).into()
                            )
                        }
                    }
                } else {
                    return Err(format!("Parser: Expected Char got: {:?}", curr_token).into());
                }

                i += 2;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                match result.node {
                    Node::Program(body) => ast.push(Node::ForLoop {
                        var_name,
                        start,
                        end,
                        body,
                    }),
                    _ => {
                        return Err(format!("Parser: Could not parse body: {:?}", curr_token).into())
                    }
                }
                i = result.end_num + 1;

                continue;
            }
        }
        match curr_token {
            Token::BracketClose(_) => {
                return Ok(Output {
                    node: Node::Program(ast),
                    end_num: i,
                })
            }
            _ => return Err(format!("Parser: Could not parse: {:?}", curr_token).into()),
        }
    }
    Ok(Output {
        node: Node::Program(ast),
        end_num: i,
    })
}
