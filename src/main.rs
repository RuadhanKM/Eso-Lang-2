use std::{io::prelude::*, sync::Mutex, collections::HashMap};
use libloading::Library;
use clap::Parser;

pub mod esotypes;
use esotypes::*;

lazy_static::lazy_static! {
    static ref USER_VAR_START: regex::Regex = regex::Regex::new(r"[A-Za-z_]").unwrap();
    static ref USER_VAR_END: regex::Regex = regex::Regex::new(r"[^A-Za-z1-9_]").unwrap();
    static ref NUMBER_START: regex::Regex = regex::Regex::new(r"[1-9]").unwrap();
    static ref NUMBER_END: regex::Regex = regex::Regex::new(r"[^1-9]").unwrap();

    static ref VARS: Mutex<HashMap<String, Var>> = Mutex::new(HashMap::new());
    static ref LIBRARIES: Mutex<HashMap<String, Library>> = Mutex::new(HashMap::new());
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
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
    let mut i = 0;
    let mut val = None;

    while i < token_stack.len(){
        let v = &token_stack[i];

        println!("{}", v.to_string());

        match v.token_type {
            TokenType::Operator => {
                match v.token_value.as_ref().unwrap().as_str() {
                    "#" => {
                        if token_stack.get(i+1).is_some() {
                            let s: String = token_stack[i+1].token_value.clone().unwrap();
                            unsafe {
                                LIBRARIES.lock().unwrap().insert(
                                    s.clone(), 
                                    libloading::Library::new(s.clone()).unwrap()
                                );
                            }
                        }
                        i += 1;
                    }
                    _ => ()
                }
                i += 1;
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
                        // Search for user-defined function
                        let mut found = false;
                        for (name, var) in VARS.lock().unwrap().iter() {
                            if *name == *v.token_value.as_ref().unwrap() {
                                evaluate_tokens(var.value.token_nested.as_ref().unwrap());
                                found = true;
                            }
                        }
                        // Search for external library function
                        if !found {
                            for (_, library) in LIBRARIES.lock().unwrap().iter() {
                                unsafe {
                                    let f = library.get::<unsafe extern fn(&Vec<Token>) -> Option<Token>>(v.token_value.as_ref().unwrap().as_bytes());
                                    let params = token_stack[i+1].token_nested.as_ref().unwrap().split(|x| {if matches!(x.token_type, TokenType::Operator) {return x.token_value.as_ref().unwrap() == ",";} false});
                                    
                                    for a in params {
                                        println!("FUNC PARAMS: {}", evaluate_tokens(&a.to_vec()).unwrap().to_string());
                                    }

                                    if f.is_ok() {
                                        return f.unwrap()(token_stack[i+1].token_nested.as_ref().unwrap());
                                    }
                                }
                            }
                        }
                        i += 1;
                    }
                    _ => ()
                }
                i += 1;
            }
            _ => i += 1
        }
    }

    val
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
