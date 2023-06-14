pub mod esotypes;
use esotypes::*;

#[no_mangle]
pub extern "C" fn print(tokens: &Vec<Option<Var>>) -> Option<Var> {
    let mut s = String::new();

    for token in tokens {
        s += token.as_ref().unwrap().value.token_value.clone().unwrap().as_str();
        s += " ";
    }

    println!("{}", s);
    None
}