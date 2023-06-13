#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: Option<String>,
    pub token_nested: Option<Vec<Token>>
}

pub struct Var {
    pub datatype: Datatype,
    pub value: Token
}

pub enum Datatype {
    Func,
    Class,
    String,
    Number
}

#[derive(Clone)]
pub enum TokenType {
    UserVar,
    String,
    Parenthetical,
    Block,
    Number,
    Operator
}

pub enum Operator {
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
    Comma,
    Semicolon,
    Return,
    Import,
    Comment
}

impl Operator {
    pub fn get_value(c: &char) -> Option<Operator> {
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
            ',' => Some(Operator::Comma),
            '.' => Some(Operator::Dot),
            ';' => Some(Operator::Semicolon),
            '^' => Some(Operator::Return),
            '#' => Some(Operator::Import),
            '\\' => Some(Operator::Comment),
            _ => None
        }
    }
    pub fn get_char(&self) -> char {
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
            Operator::Comma => ',',
            Operator::Dot => '.',
            Operator::Semicolon => ';',
            Operator::Return => '^',
            Operator::Import => '#',
            Operator::Comment => '\\'
        }
    }
}

impl Token {
    pub fn new_value(token_type: TokenType, token_value: String) -> Token {
        Token { token_type, token_value: Some(token_value), token_nested: None }
    }
    pub fn new_nested(token_type: TokenType, nested_value: Vec<Token>) -> Token {
        Token { token_type, token_value: None, token_nested: Some(nested_value) }
    }
    pub fn to_string(&self) -> &str {
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