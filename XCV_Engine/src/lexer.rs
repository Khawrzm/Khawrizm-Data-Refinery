#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    CellRef(String),
    Function(String),
    Operator(char),
    LeftParen,
    RightParen,
    Comma,
}

pub fn tokenize(formula: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = formula.trim_start_matches('=').chars().peekable();
    
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' { num_str.push(chars.next().unwrap()); } 
                    else { break; }
                }
                tokens.push(Token::Number(num_str.parse().unwrap_or(0.0)));
            },
            'A'..='Z' | 'a'..='z' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() { ident.push(chars.next().unwrap()); } 
                    else { break; }
                }
                if let Some(&'(') = chars.peek() { tokens.push(Token::Function(ident.to_uppercase())); } 
                else { tokens.push(Token::CellRef(ident.to_uppercase())); }
            },
            '+' | '-' | '*' | '/' | '^' => { tokens.push(Token::Operator(chars.next().unwrap())); },
            '(' => { tokens.push(Token::LeftParen); chars.next(); },
            ')' => { tokens.push(Token::RightParen); chars.next(); },
            ',' => { tokens.push(Token::Comma); chars.next(); },
            ' ' => { chars.next(); }, // تجاهل المسافات
            _ => { chars.next(); } // تجاهل الباقي مؤقتاً
        }
    }
    tokens
}
