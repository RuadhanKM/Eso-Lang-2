pub mod esotypes;
use esotypes::*;

#[no_mangle]
pub extern "C" fn print(tokens: &Vec<Token>) -> Option<Token> {
    let mut s = String::new();

    for token in tokens.clone() {
        s += token.token_value.unwrap().as_str();
    }

    println!("{}", s);
    None
}