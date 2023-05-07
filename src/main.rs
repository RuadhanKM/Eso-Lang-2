use std::{io::prelude::*, collections::HashMap};
use clap::Parser;
use libloading::Library;

lazy_static::lazy_static! {
    static ref USER_VAR_START: regex::Regex = regex::Regex::new(r"[A-Za-z_]").unwrap();
    static ref USER_VAR_END: regex::Regex = regex::Regex::new(r"[^A-Za-z1-9_]").unwrap();
    static ref NUMBER_START: regex::Regex = regex::Regex::new(r"[1-9]").unwrap();
    static ref NUMBER_END: regex::Regex = regex::Regex::new(r"[^1-9]").unwrap();

    static ref VARS: HashMap<String, Var> = HashMap::new();
    static ref LIBRARIES: HashMap<String, Library> = HashMap::new();
}


#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

struct Token {
    token_type: TokenType,
    token_value: Option<String>,
    token_nested: Option<Vec<Token>>
}

struct Var {
    datatype: Datatype,
    value: Token
}

enum Datatype {
    Func,
    Class,
    String,
    Number
}

enum TokenType {
    UserVar,
    String,
    Parenthetical,
    Block,
    Number,
    Operator
}

enum Operator {
    Def,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Less,
    Greater,
    Equal,
    Not,
    Dot,
    Semicolon,
    Return,
    Import,
    Comment
}

impl Operator {
    fn get_value(c: &char) -> Option<Operator> {
        match c {
            '=' => Some(Operator::Def),
            '&' => Some(Operator::And),
            '|' => Some(Operator::Or),
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            '<' => Some(Operator::Less),
            '>' => Some(Operator::Greater),
            '~' => Some(Operator::Equal),
            '!' => Some(Operator::Not),
            '.' => Some(Operator::Dot),
            ';' => Some(Operator::Semicolon),
            '^' => Some(Operator::Return),
            '#' => Some(Operator::Import),
            '\\' => Some(Operator::Comment),
            _ => None
        }
    }
    fn get_char(&self) -> char {
        match self {
            Operator::Def => '=',
            Operator::And => '&',
            Operator::Or => '|',
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Mul => '*',
            Operator::Div => '/',
            Operator::Less => '<',
            Operator::Greater => '>',
            Operator::Equal => '~',
            Operator::Not => '!',
            Operator::Dot => '.',
            Operator::Semicolon => ';',
            Operator::Return => '^',
            Operator::Import => '#',
            Operator::Comment => '\\'
        }
    }
}

impl Token {
    fn new_value(token_type: TokenType, token_value: String) -> Token {
        Token { token_type, token_value: Some(token_value), token_nested: None }
    }
    fn new_nested(token_type: TokenType, nested_value: Vec<Token>) -> Token {
        Token { token_type, token_value: None, token_nested: Some(nested_value) }
    }
    fn to_string(&self) -> &str {
        match self.token_type {
            TokenType::Block => "Block",
            TokenType::Number => "Number",
            TokenType::UserVar => "Var",
            TokenType::Parenthetical => "Parenthetical",
            TokenType::String => "String",
            TokenType::Operator => "Operator"
        }
    }
}

fn substr(s: &String, start: usize, len: usize) -> String {
    s.chars().skip(start).take(len).collect()
}

fn string_to_charvec(s: &String) -> Vec<char> {
    s.chars().collect()
}

fn charvec_to_string(s: &Vec<char>) -> String {
    s.iter().collect()
}

fn tokenize_substr<F>(chars: &Vec<char>, i: &mut usize, find: F) -> Vec<char> 
    where F: FnMut(&char) -> bool
{
    let dis = chars.iter().skip(*i).position(find).unwrap_or(chars.len());
    let fin = string_to_charvec(&substr(&charvec_to_string(chars), *i, dis));
    (*i) += dis;
    fin
}

fn tokenize(chars: &Vec<char>) -> Vec<Token> {
    let mut i = 0;
    let mut token_stack: Vec<Token> = Vec::new();
    
    while i < chars.len() {
        let letter = chars[i];

        match letter {
            '(' => {
                i += 1;
                token_stack.push(
                    Token::new_nested(
                        TokenType::Parenthetical, 
                        tokenize(&tokenize_substr(chars, &mut i, |&x| x == ')'))
                    )
                );
                i += 1;
            }
            '"' => {
                i += 1;
                token_stack.push(
                    Token::new_value(
                        TokenType::String,
                        charvec_to_string(&tokenize_substr(chars, &mut i, |&x| x == '"'))
                    )
                );
                i += 1;
            }
            c if USER_VAR_START.is_match(c.encode_utf8(&mut [0; 4])) => {
                token_stack.push(
                    Token::new_value(
                        TokenType::UserVar,
                        charvec_to_string(
                            &tokenize_substr(
                                chars, 
                                &mut i, 
                                |&x| USER_VAR_END.is_match(x.encode_utf8(&mut [0; 4]))
                            )
                        )
                    )
                );
            }
            c if NUMBER_START.is_match(c.encode_utf8(&mut [0; 4])) => {
                token_stack.push(
                    Token::new_value(
                        TokenType::Number, 
                        charvec_to_string(
                            &tokenize_substr(
                                chars, 
                                &mut i, 
                                |&x| NUMBER_END.is_match(x.encode_utf8(&mut [0; 4]))
                            )
                        )
                    )
                );
            }
            '{' => {
                i += 1;
                token_stack.push(
                    Token::new_nested(
                        TokenType::Block,
                        tokenize(&tokenize_substr(chars, &mut i, |&x| x == '}'))
                    )
                );
                i += 1;
            }
            c if Operator::get_value(&c).is_some() => {
                token_stack.push(Token::new_value(TokenType::Operator, c.to_string()));
                i += 1;
            }
            _ => i += 1
        };
    }
    
    token_stack
}

fn print_tokens(token_stack: &Vec<Token>, depth: usize) {
    for i in token_stack {
        if i.token_nested.is_some() {
            println!("{}", i.to_string());
            print_tokens(i.token_nested.as_ref().unwrap(), depth+1);
        } else if i.token_value.is_some() {
            println!("{}{}: \"{}\"", std::iter::repeat("  ").take(depth).collect::<String>(), i.to_string(), i.token_value.as_ref().unwrap());
        } else {
            println!("{}{}", std::iter::repeat("  ").take(depth).collect::<String>(), i.to_string());
        }
    }
}

fn evaluate_tokens(token_stack: &Vec<Token>) -> Option<Token> {
    for (i, v) in token_stack.iter().enumerate() {
        match v.token_type {
            TokenType::Operator => {
                match v.token_value.as_ref().unwrap().as_str() {
                    "#" => {
                        if token_stack.get(i+1).is_some() {
                            let s: String = token_stack[i+1].token_value.clone().unwrap();
                            unsafe {
                                let l = libloading::Library::new(s.clone()).unwrap();
                                LIBRARIES.insert(
                                    s.clone(), 
                                    l
                                );
                            }
                        }
                    }
                    _ => ()
                }
            }
            TokenType::Block => {
                if token_stack.get(i-1).is_some() {
                    match token_stack[i-1].token_type {
                        TokenType::Operator => {
                            match token_stack[i-1].token_value.as_ref().unwrap().as_str() {
                                "=" => {

                                } 
                                _ => ()
                            }
                        },
                        TokenType::Parenthetical => {

                        }
                        _ => ()
                    }
                } else {
                    evaluate_tokens(v.token_nested.as_ref().unwrap());
                }
            },
            TokenType::UserVar => {
                match token_stack[i+1].token_type {
                    TokenType::Parenthetical => {
                        let mut found = false;
                        for name in VARS.keys() {
                            if *name == *v.token_value.as_ref().unwrap() {
                                evaluate_tokens(VARS.get(name).unwrap().value.token_nested.as_ref().unwrap());
                                found = true;
                            }
                        }
                        if !found {
                            for (name, library) in LIBRARIES.iter() {
                                if *name == *v.token_value.as_ref().unwrap() {
                                    unsafe {
                                        library.get::<unsafe extern fn(&Token) -> Token>(name.as_bytes()).unwrap()(v);
                                    }
                                }
                            }
                        }
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = Cli::parse();
    
    let mut file = std::fs::File::open(args.path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let chars = file_content.chars().collect::<Vec<char>>();
    
    let token_stack: Vec<Token> = tokenize(&chars);

    print_tokens(&token_stack, 0);

    evaluate_tokens(&token_stack);

    Ok(())
}
