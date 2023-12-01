use std::str::FromStr;

pub struct Tokenizer {
    tokens: Vec<Token>,
    first_advance: bool,
    cur_token_i: usize,
}

struct Token {
    token_type: TokenType,
    keyword: Keyword,
    symbol: char,
    identifier: String,
    int_val: i32,
    string_val: String,
    comment: String,
}

enum TokenFoo {
    Keyword { t: TokenType, keyword: String },
    Symbol { t: TokenType, keyword: String },
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
    LineComment,
    BlockComment,
}

#[derive(Clone, Copy)]
pub enum Keyword {
    None,
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Class => "class".to_string(),
            Self::Method => "method".to_string(),
            Self::Function => "function".to_string(),
            Self::Constructor => "constructor".to_string(),
            Self::Int => "int".to_string(),
            Self::Boolean => "boolean".to_string(),
            Self::Char => "char".to_string(),
            Self::Void => "void".to_string(),
            Self::Var => "var".to_string(),
            Self::Static => "static".to_string(),
            Self::Field => "field".to_string(),
            Self::Let => "let".to_string(),
            Self::Do => "do".to_string(),
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),
            Self::While => "while".to_string(),
            Self::Return => "return".to_string(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Null => "null".to_string(),
            Self::This => "this".to_string(),
        }
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "class" => Ok(Self::Class),
            "constructor" => Ok(Self::Constructor),
            "function" => Ok(Self::Function),
            "method" => Ok(Self::Method),
            "field" => Ok(Self::Field),
            "static" => Ok(Self::Static),
            "var" => Ok(Self::Var),
            "int" => Ok(Self::Int),
            "char" => Ok(Self::Char),
            "boolean" => Ok(Self::Boolean),
            "void" => Ok(Self::Void),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "null" => Ok(Self::Null),
            "this" => Ok(Self::This),
            "let" => Ok(Self::Let),
            "do" => Ok(Self::Do),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "while" => Ok(Self::While),
            "return" => Ok(Self::Return),
            _ => Err(()),
        }
    }
}

impl Tokenizer {
    pub fn new(code_contents: String, ignore_comments: bool) -> Tokenizer {
        let tokens = Tokenizer::analyze(&code_contents, ignore_comments);

        Tokenizer {
            tokens,
            first_advance: true,
            cur_token_i: 0,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        return self.cur_token_i < self.tokens.len();
    }

    pub fn advance(&mut self) {
        if !self.first_advance {
            self.cur_token_i += 1;
        }
        self.first_advance = false;
    }

    pub fn token_type(&self) -> TokenType {
        self.tokens[self.cur_token_i].token_type
    }

    pub fn keyword(&self) -> Keyword {
        self.tokens[self.cur_token_i].keyword
    }

    pub fn symbol(&self) -> char {
        self.tokens[self.cur_token_i].symbol
    }

    pub fn identifier(&self) -> String {
        self.tokens[self.cur_token_i].identifier.to_owned()
    }

    pub fn int_val(&self) -> i32 {
        self.tokens[self.cur_token_i].int_val
    }

    pub fn string_val(&self) -> String {
        self.tokens[self.cur_token_i].string_val.to_owned()
    }

    pub fn comment_val(&self) -> String {
        self.tokens[self.cur_token_i].comment.to_owned()
    }

    fn analyze(code: &str, ignore_comments: bool) -> Vec<Token> {
        let mut tokens = Vec::new();

        let keywords: [&str; 21] = [
            "class",
            "constructor",
            "function",
            "method",
            "field",
            "static",
            "var",
            "int",
            "char",
            "boolean",
            "void",
            "true",
            "false",
            "null",
            "this",
            "let",
            "do",
            "if",
            "else",
            "while",
            "return",
        ];
        let symbols: [char; 19] = [
            '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>',
            '=', '~',
        ];

        let mut character_stream = code.chars();
        let mut cur = character_stream.next();

        while cur != None {
            let c = cur.unwrap();
            if c == '/' {
                cur = character_stream.next();
                if cur == None {
                    break;
                }

                let mut comment: String = "".to_owned();
                let mut c = cur.unwrap();
                if c == '/' {
                    while c != '\n' {
                        comment.push(c);
                        cur = character_stream.next();
                        c = cur.unwrap();
                    }
                    if !ignore_comments {
                        tokens.push(Token {
                            token_type: TokenType::LineComment,
                            keyword: Keyword::None,
                            symbol: ' ',
                            identifier: String::new(),
                            int_val: 0,
                            string_val: String::new(),
                            comment,
                        });
                    }
                } else if c == '*' {
                    let mut prev_c = ' ';
                    loop {
                        if prev_c == '*' && c == '/' {
                            cur = character_stream.next();
                            break;
                        }

                        comment.push(c);
                        cur = character_stream.next();

                        prev_c = c;
                        c = cur.unwrap();
                    }
                    if !ignore_comments {
                        tokens.push(Token {
                            token_type: TokenType::BlockComment,
                            keyword: Keyword::None,
                            symbol: ' ',
                            identifier: String::new(),
                            int_val: 0,
                            string_val: String::new(),
                            comment,
                        });
                    }
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Symbol,
                        keyword: Keyword::None,
                        symbol: '/',
                        identifier: String::new(),
                        int_val: 0,
                        string_val: String::new(),
                        comment: String::new(),
                    })
                }
                continue;
            } else if c.is_alphabetic() {
                let mut token: String = "".to_owned();
                let mut c = cur.unwrap();
                while c != ' ' && !symbols.contains(&c) {
                    token.push(c);
                    cur = character_stream.next();
                    c = cur.unwrap();
                }

                if keywords.contains(&token.as_str()) {
                    tokens.push(Token {
                        token_type: TokenType::Keyword,
                        keyword: Keyword::from_str(&token).unwrap_or(Keyword::None),
                        symbol: ' ',
                        identifier: String::new(),
                        int_val: 0,
                        string_val: String::new(),
                        comment: String::new(),
                    })
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Identifier,
                        keyword: Keyword::None,
                        symbol: ' ',
                        identifier: token,
                        int_val: 0,
                        string_val: String::new(),
                        comment: String::new(),
                    })
                }
                continue;
            } else if symbols.contains(&c) {
                tokens.push(Token {
                    token_type: TokenType::Symbol,
                    keyword: Keyword::None,
                    symbol: c,
                    identifier: String::new(),
                    int_val: 0,
                    string_val: String::new(),
                    comment: String::new(),
                });
                cur = character_stream.next();
                continue;
            } else if c.is_numeric() {
                let mut token: String = "".to_owned();
                let mut c = cur.unwrap();
                while c.is_numeric() {
                    token.push(c);
                    cur = character_stream.next();
                    c = cur.unwrap();
                }

                tokens.push(Token {
                    token_type: TokenType::IntConst,
                    keyword: Keyword::None,
                    symbol: ' ',
                    identifier: String::new(),
                    int_val: i32::from_str(&token).unwrap_or(0),
                    string_val: String::new(),
                    comment: String::new(),
                });
                continue;
            } else if c == '"' {
                cur = character_stream.next();
                let mut token: String = "".to_owned();
                let mut c = cur.unwrap();

                while c != '"' {
                    token.push(c);
                    cur = character_stream.next();
                    c = cur.unwrap();
                }
                cur = character_stream.next();

                tokens.push(Token {
                    token_type: TokenType::StringConst,
                    keyword: Keyword::None,
                    symbol: ' ',
                    identifier: String::new(),
                    int_val: 0,
                    string_val: token,
                    comment: String::new(),
                });
                continue;
            }

            cur = character_stream.next();
        }

        tokens
    }
}
