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
    DoubleDot,
    Colon,
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
            ',' => {
                tokens.push(Token::Symbol(Symbol::Colon));
                i += 1;
                continue;
            }
            ';' => {
                tokens.push(Token::Symbol(Symbol::SemiColon));
                i += 1;
                continue;
            }
            ':' => {
                tokens.push(Token::Symbol(Symbol::DoubleDot));
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
    StringLiteral(String),
    NumberLiteral(String),
    Symbol(Symbol),
    BracketOpen(Bracket),
    BracketClose(Bracket),
    NewLine,
    VeriableCall(String),
    Veriable {
        name: String,
        value: Vec<Node>,
    },
    IfStatement {
        condition: Vec<Node>,
        body: Vec<Node>,
    },
    ElseStatement {
        body: Vec<Node>,
    },
    ForLoop {
        name: String,
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
    pub node: Vec<Node>,
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
                let mut value = Vec::new();
                i += 1;
                curr_token = &tokens[i];
                if let Token::Char(b) = curr_token {
                    name = b.to_string();
                } else {
                    return Err(format!(
                        "Parser Variable: Expected char got: {:?} at {i}",
                        curr_token
                    )
                    .into());
                }

                i += 1;
                curr_token = &tokens[i];

                while !matches!(*curr_token, Token::Symbol(Symbol::SemiColon)) {
                    match curr_token {
                        Token::Char(n) => value.push(Node::VeriableCall(n.to_string())),
                        Token::Number(n) => value.push(Node::NumberLiteral(n.to_string())),
                        Token::String(s) => value.push(Node::NumberLiteral(s.to_string())),
                        Token::Symbol(s) => value.push(Node::Symbol(*s)),
                        Token::BracketOpen(b) => value.push(Node::BracketOpen(b.clone())),
                        Token::BracketClose(b) => value.push(Node::BracketClose(b.clone())),
                    }
                    i += 1;
                    curr_token = &tokens[i];
                }
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
                                "Parser If statement: Expected Char/String/Number got: {:?} at {i}",
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
                ast.push(Node::IfStatement {
                    condition,
                    body: result.node,
                });
                i = result.end_num + 1;

                continue;
            }

            if a == "else" {
                i += 2;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                ast.push(Node::ElseStatement { body: result.node });
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
                                "Parser While loop: Expected Char/String/Number got: {:?} at {i}",
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
                ast.push(Node::WhileLoop {
                    condition,
                    body: result.node,
                });
                i = result.end_num + 1;

                continue;
            }

            if a == "for" {
                // Create the condition
                i += 1;
                curr_token = &tokens[i];
                let name;
                let start;
                if let Token::Char(c) = curr_token {
                    name = c.to_string();
                } else {
                    return Err(format!(
                        "Parser For loop: Expected Char got: {:?} at {i}",
                        curr_token
                    )
                    .into());
                }

                i += 2;
                curr_token = &tokens[i];

                match curr_token {
                    Token::Number(n) => start = n.to_string(),
                    Token::Char(c) => start = c.to_string(),
                    _ => {
                        return Err(format!(
                            "Parser For loop: Expected Num/Char got: {:?} at {i}",
                            curr_token
                        )
                        .into())
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
                            return Err(format!(
                                "Parser For loop: Expected Num/Char got: {:?} at {i}",
                                curr_token
                            )
                            .into())
                        }
                    }
                } else if let Token::Number(_) = curr_token {
                    match curr_token {
                        Token::Number(n) => end = n.to_string(),
                        Token::Char(c) => end = c.to_string(),
                        _ => {
                            return Err(format!(
                                "Parser For loop: Expected Num/Char got: {:?} at {i}",
                                curr_token
                            )
                            .into())
                        }
                    }
                } else {
                    return Err(format!(
                        "Parser For loop: Expected Char got: {:?} at {i}",
                        curr_token
                    )
                    .into());
                }

                i += 2;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                ast.push(Node::ForLoop {
                    name,
                    start,
                    end,
                    body: result.node,
                });
                i = result.end_num + 1;

                continue;
            }

            if a == "fn" {
                // Create the condition
                i += 1;
                curr_token = &tokens[i];
                let name;
                if let Token::Char(c) = curr_token {
                    name = c.to_string();
                } else {
                    return Err(format!(
                        "Parser Function: Expected Char got: {:?} at {i}",
                        curr_token
                    )
                    .into());
                }

                i += 2;
                curr_token = &tokens[i];

                let mut input = Vec::new();
                while !matches!(*curr_token, Token::BracketOpen(Bracket::Curly)) {
                    curr_token = &tokens[i];
                    match curr_token {
                        Token::Char(c) => input.push(Node::VeriableCall(c.to_string())),
                        Token::Symbol(Symbol::Colon) => {
                            i += 1;
                            continue;
                        }
                        Token::BracketClose(Bracket::Paren) => {
                            i += 1;
                            break;
                        }
                        _ => {
                            return Err(format!(
                                "Parser Function: Expected Char/Colon/ParenOpen got: {:?} at {i}",
                                curr_token
                            )
                            .into())
                        }
                    }
                    i += 3;
                }

                i += 1;
                // Creat the body of the statement
                let result = parser(i, tokens.to_vec())?;
                ast.push(Node::Function {
                    name,
                    input,
                    body: result.node,
                });
                i = result.end_num + 1;

                continue;
            }

            // Handle Function calls
            if let Token::BracketOpen(Bracket::Paren) = tokens[i + 1] {
                let name;
                match curr_token {
                    Token::Char(c) => name = c.to_string(),
                    _ => {
                        return Err(format!(
                            "Parser Function call: Expected Char got: {:?} at {i}",
                            curr_token
                        )
                        .into())
                    }
                }

                i += 2;
                curr_token = &tokens[i];

                let mut input = Vec::new();
                while !matches!(*curr_token, Token::BracketClose(Bracket::Paren)) {
                    match curr_token {
                        Token::Char(c) => {
                            input.push(Node::VeriableCall(c.to_string()));
                            i += 1;
                        }
                        Token::Number(c) => {
                            input.push(Node::NumberLiteral(c.to_string()));
                            i += 1;
                        }
                        Token::String(c) => {
                            input.push(Node::StringLiteral(c.to_string()));
                            i += 1;
                        }
                        Token::Symbol(Symbol::Colon) => i += 1,
                        Token::BracketClose(Bracket::Paren) => i += 1,
                        _ => {
                            return Err(format!(
                                "Parser Function call: Expected Char/Number/String/Colon/ParenOpen got: {:?} at {i}",
                                curr_token
                            )
                            .into())
                        }
                    }
                    curr_token = &tokens[i]
                }
                i += 1;
                ast.push(Node::FunctionCall { name, input });
                continue;
            }
            // Handle var calls
            else {
                ast.push(Node::VeriableCall(a.to_string()));
                i += 1;
                curr_token = &tokens[i];

                if let Token::Symbol(Symbol::Equals) = curr_token {
                    ast.push(Node::Symbol(Symbol::Equals));
                    i += 1;
                    curr_token = &tokens[i];
                    while !matches!(*curr_token, Token::Symbol(Symbol::SemiColon)) {
                        match curr_token {
                            Token::Char(n) => ast.push(Node::VeriableCall(n.to_string())),
                            Token::Number(n) => ast.push(Node::NumberLiteral(n.to_string())),
                            Token::String(s) => ast.push(Node::NumberLiteral(s.to_string())),
                            Token::Symbol(s) => ast.push(Node::Symbol(*s)),
                            Token::BracketOpen(b) => ast.push(Node::BracketOpen(b.clone())),
                            Token::BracketClose(b) => ast.push(Node::BracketClose(b.clone())),
                        }
                        i += 1;
                        curr_token = &tokens[i];
                    }
                }
                continue;
            }
        }

        match curr_token {
            Token::Symbol(Symbol::SemiColon) => {
                ast.push(Node::NewLine);
                i += 1
            }
            Token::BracketClose(Bracket::Curly) => {
                return Ok(Output {
                    node: ast,
                    end_num: i,
                })
            }
            _ => return Err(format!("Parser: Could not parse: {:?} at {i}", curr_token).into()),
        }
    }
    return Ok(Output {
        node: ast,
        end_num: i,
    });
}
////////////////////
///CODE GENERATOR///
////////////////////

pub fn code_generator(ast: Vec<Node>) -> Result<String, Box<dyn Error>> {
    let mut program = String::new();
    for i in ast {
        match i {
            Node::Veriable { name, value } => {
                program.push_str(format!("let {}{}", name, code_generator(value)?).as_str())
            }
            Node::VeriableCall(c) => program.push_str(format!("{c}").as_str()),
            Node::IfStatement { condition, body } => program.push_str(
                format!(
                    "if ({}){{\n{}}}\n",
                    code_generator(condition)?,
                    code_generator(body)?
                )
                .as_str(),
            ),
            Node::ElseStatement { body } => {
                program.push_str(format!("else{{\n{}}}\n", code_generator(body)?).as_str())
            }
            Node::WhileLoop { condition, body } => program.push_str(
                format!(
                    "while ({}){{\n{}}}\n",
                    code_generator(condition)?,
                    code_generator(body)?
                )
                .as_str(),
            ),
            Node::ForLoop {
                name,
                start,
                end,
                body,
            } => program.push_str(
                format!(
                    "for (let {name}={start}; {name}==={end}; {name}++){{\n{}}}\n",
                    code_generator(body)?
                )
                .as_str(),
            ),
            Node::FunctionCall { name, input } => {
                program.push_str(format!("{name}(").as_str());
                let mut i = 0;
                let len = input.len();
                for n in input {
                    if let Node::VeriableCall(call) = n {
                        if i > 0 {
                            program.push_str(" ")
                        }
                        program.push_str(format!("{call}").as_str());
                        if i < len - 1 {
                            program.push_str(",")
                        }
                    } else {
                        return Err(format!(
                            "Code generator Function call: Expected Variable call got: {:?}",
                            n
                        )
                        .into());
                    };
                    i += 1;
                }
                program.push_str(")");
            }
            Node::Function { name, input, body } => {
                program.push_str(format!("function {name}(").as_str());
                let mut i = 0;
                let len = input.len();
                for n in input {
                    if let Node::VeriableCall(call) = n {
                        if i > 0 {
                            program.push_str(" ")
                        }
                        program.push_str(format!("{call}").as_str());
                        if i < len - 1 {
                            program.push_str(",")
                        }
                    } else {
                        return Err(format!(
                            "Code generator Function: Expected Variable call got: {:?}",
                            n
                        )
                        .into());
                    };
                    i += 1;
                }
                program.push_str(format!("){{\n{}}}\n", code_generator(body)?).as_str());
            }
            Node::NewLine => program.push_str(";\n"),
            Node::StringLiteral(s) => program.push_str(format!("\"{s}\"").as_str()),
            Node::NumberLiteral(n) => program.push_str(format!("{n}").as_str()),
            Node::Symbol(s) => match s {
                Symbol::Equals => program.push_str("="),
                Symbol::Plus => program.push_str("+"),
                Symbol::Minus => program.push_str("-"),
                Symbol::Multiply => program.push_str("*"),
                Symbol::Devide => program.push_str("/"),
                Symbol::Modulo => program.push_str("%"),
                Symbol::Grater => program.push_str(">"),
                Symbol::Lesser => program.push_str("<"),
                Symbol::Dot => program.push_str("."),
                Symbol::SemiColon => program.push_str(";"),
                Symbol::Exclamation => program.push_str("!"),
                Symbol::DoubleDot => program.push_str(":"),
                Symbol::Colon => program.push_str(","),
            },
            Node::BracketOpen(b) => match b {
                Bracket::Curly => program.push_str("{"),
                Bracket::Square => program.push_str("["),
                Bracket::Paren => program.push_str("("),
            },
            Node::BracketClose(b) => match b {
                Bracket::Curly => program.push_str("}"),
                Bracket::Square => program.push_str("]"),
                Bracket::Paren => program.push_str(")"),
            },
        }
    }
    Ok(program)
}
