use std::{iter::Peekable, str::Chars};

use super::token::{Token, TokenType, Value, Position, Operator, Keyword};

#[derive(Debug, Clone)]
pub struct LexingError {
    pub message: String,
    pub position: Position,
}

impl LexingError {
    fn new(message: String, position: Position) -> Self {
        Self { message, position }
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: Position,
    current_char: Option<char>,
    errors: Vec<LexingError>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input : input.chars().peekable(),
            position: Position { row: 0, col: 0 },
            current_char: None,
            errors: vec![],
        };
        lexer.advance();
        lexer
    }

    /// Moves the lexer to the next character in the input file.
    fn advance(&mut self) -> () {

        // Move to next character.
        if let Some(c) = self.input.next() {

            // Update current_char & position, and make sure newlines don't ruin the position field.
            self.current_char = Some(c);
            self.position.col += 1;
            if c == '\n' {
                self.position.col = 0;
                self.position.row += 1;
            }

            // If the definition doesn't work, probably at EOF so just set to None.
        } else {
            self.current_char = None;
        }
    }

    /// Provides the next token in a stream of characters.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Define the token by checking the current character, and then handle.
        let token: Token = match self.current_char {

            // Pattern match for different tokens.
            Some(c) => match c {
                '/' => self.handle_comment_or_division(),
                'a'..='z' | 'A'..='Z' | '_' => self.handle_symbol(),
                '0'..='9' => self.handle_number(),
                '"' => self.handle_string(),
                '\'' => self.handle_char(),
                '+' | '-' | '*' | '=' | '<' | '>' | '&' | '|' => self.handle_operator(),
                ';' | ',' | '(' | ')' | '{' | '}' | '[' | ']' | '.' => self.handle_punctuator(),

                // If there is no handler for the character, push an error with some info to the error list.
                // Also, make sure the lexer still continues it's advancement.
                _ => {
                    let error_position = self.position.clone();
                    self.errors.push(LexingError::new(
                        format!("Unexpected character: {}", c),
                        error_position
                    ));

                    self.advance();
                    return self.next_token()
                }
            },
            None => return Token::new(TokenType::EOF, Value::Empty, self.position.row, self.position.col),
        };
        token
    }

    fn handle_char(&mut self) -> Token {
        let position = self.position.clone();
        let mut value = String::new();

        self.advance();

        while let Some(c) = self.current_char {
            if value.len() == 1 {
                break;
            }
            if c != '\'' {
                value.push(c);
                self.advance();
            } else {
                return Token::new(TokenType::EmptyLiteral, Value::Empty, position.row, position.col)
            }
        }
        self.advance();
        Token::new(TokenType::CharLiteral, Value::Char(value.chars().next().unwrap()), position.row, position.col)
    }

    fn handle_comment_or_division(&mut self) -> Token {
        let position = self.position.clone();
        let mut value = String::new();

        self.advance();
        if self.current_char.unwrap() == '/' {
            self.advance();
            while let Some(c) = self.current_char {
                if c == '\n' {
                    break;
                }
                value.push(c);
                self.advance();
            }
            return Token::new(TokenType::Comment, Value::String(value), position.row, position.col)
        }
        if self.current_char.unwrap() == '*' {
            self.advance();
            while let Some(c) = self.current_char {
                if c == '*' {
                    if self.input.next().unwrap() == '/' {
                        break;
                    }
                }
                value.push(c);
                self.advance();
            }
            self.advance();
            return Token::new(TokenType::Comment, Value::String(value), position.row, position.col)
        }

        return Token::new(TokenType::Operator(Operator::Divide), Value::String(value), position.row, position.col)
    }

    fn handle_operator(&mut self) -> Token {
        let position = self.position.clone();

        let mut value = String::new();

        while let Some(c) = self.current_char {
            if value.len() == 2 && (value != "<<" || value != ">>") {
                break;
            }
            if !c.is_alphanumeric() && !c.is_whitespace() {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let token_type = match value.as_str() {
            // Arithmetic
            "+" => TokenType::Operator(Operator::Add),
            "-" => TokenType::Operator(Operator::Subtract),
            "*" => TokenType::Operator(Operator::MultiplyOrPointer),
            "/" => TokenType::Operator(Operator::Divide),
            "%" => TokenType::Operator(Operator::Modulo),
            "++" => TokenType::Operator(Operator::Increment),
            "--" => TokenType::Operator(Operator::Decrement),

            // Bitwise
            "&" => TokenType::Operator(Operator::BitwiseAndOrDereference),
            "|" => TokenType::Operator(Operator::BitwiseOr),
            "^" => TokenType::Operator(Operator::BitwiseXor),
            "~" => TokenType::Operator(Operator::BitwiseNot),
            "<<" => TokenType::Operator(Operator::LeftShift),
            ">>" => TokenType::Operator(Operator::RightShift),

            // Logical
            "&&" => TokenType::Operator(Operator::LogicalAnd),
            "||" => TokenType::Operator(Operator::LogicalOr),
            "!" => TokenType::Operator(Operator::LogicalNot),

            // Comparison
            "==" => TokenType::Operator(Operator::Equal),
            "!=" => TokenType::Operator(Operator::NotEqual),
            "<" => TokenType::Operator(Operator::LessThan),
            ">" => TokenType::Operator(Operator::GreaterThan),
            "<=" => TokenType::Operator(Operator::LessThanOrEqual),
            ">=" => TokenType::Operator(Operator::GreaterThanOrEqual),

            // Assignment
            "=" => TokenType::Operator(Operator::Assign),
            "+=" => TokenType::Operator(Operator::AddAssign),
            "-=" => TokenType::Operator(Operator::SubtractAssign),
            "*=" => TokenType::Operator(Operator::MultiplyAssign),
            "/=" => TokenType::Operator(Operator::DivideAssign),
            "%=" => TokenType::Operator(Operator::ModuloAssign),
            "&=" => TokenType::Operator(Operator::BitwiseAndAssign),
            "|=" => TokenType::Operator(Operator::BitwiseOrAssign),
            "^=" => TokenType::Operator(Operator::BitwiseXorAssign),
            "<<=" => TokenType::Operator(Operator::LeftShiftAssign),
            ">>=" => TokenType::Operator(Operator::RightShiftAssign),

            _ => TokenType::Unknown,
        };

        Token::new(token_type, Value::String(value), position.row, position.col)
    }

    fn handle_punctuator(&mut self) -> Token {
        // Get relevant info
        let position = self.position.clone();
        let value = self.current_char.unwrap();
        let token_type = TokenType::Punctuator(value);

        self.advance();
        Token::new(token_type, Value::Char(value), position.row, position.col)
    }

    fn handle_symbol(&mut self) -> Token {
        let token_position = self.position.clone();

        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match identifier.as_str() {
            "auto" => TokenType::Keyword(Keyword::Auto),
            "break" => TokenType::Keyword(Keyword::Break),
            "case" => TokenType::Keyword(Keyword::Case),
            "char" => TokenType::Keyword(Keyword::Char),
            "const" => TokenType::Keyword(Keyword::Const),
            "continue" => TokenType::Keyword(Keyword::Continue),
            "default" => TokenType::Keyword(Keyword::Default),
            "do" => TokenType::Keyword(Keyword::Do),
            "double" => TokenType::Keyword(Keyword::Double),
            "else" => TokenType::Keyword(Keyword::Else),
            "enum" => TokenType::Keyword(Keyword::Enum),
            "extern" => TokenType::Keyword(Keyword::Extern),
            "float" => TokenType::Keyword(Keyword::Float),
            "for" => TokenType::Keyword(Keyword::For),
            "goto" => TokenType::Keyword(Keyword::Goto),
            "if" => TokenType::Keyword(Keyword::If),
            "int" => TokenType::Keyword(Keyword::Int),
            "long" => TokenType::Keyword(Keyword::Long),
            "register" => TokenType::Keyword(Keyword::Register),
            "return" => TokenType::Keyword(Keyword::Return),
            "short" => TokenType::Keyword(Keyword::Short),
            "signed" => TokenType::Keyword(Keyword::Signed),
            "sizeof" => TokenType::Keyword(Keyword::Sizeof),
            "static" => TokenType::Keyword(Keyword::Static),
            "struct" => TokenType::Keyword(Keyword::Struct),
            "switch" => TokenType::Keyword(Keyword::Switch),
            "typedef" => TokenType::Keyword(Keyword::Typedef),
            "union" => TokenType::Keyword(Keyword::Union),
            "unsigned" => TokenType::Keyword(Keyword::Unsigned),
            "void" => TokenType::Keyword(Keyword::Void),
            "volatile" => TokenType::Keyword(Keyword::Volatile),
            "while" => TokenType::Keyword(Keyword::While),
            _ => TokenType::Identifier,
        };

        Token::new(token_type, Value::String(identifier), token_position.row, token_position.col)
    }

    fn handle_number(&mut self) -> Token {
        let position = self.position.clone();
        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c == '.' && value.contains('.') {
                break;
            }
            if c.is_ascii_digit() || c == '.' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if value.contains('.') {
            Token::new(TokenType::FloatLiteral, Value::Float(value.parse().unwrap()), position.row, position.col)

        } else {
            Token::new(TokenType::IntegerLiteral, Value::Integer(value.parse().unwrap()), position.row, position.col)

        }

    }

    fn handle_string(&mut self) -> Token {
        let position = self.position.clone();
        let mut value = String::new();

        self.advance();

        while let Some(c) = self.current_char {
            if c != '"' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.advance();
        Token::new(TokenType::StringLiteral, Value::String(value), position.row, position.col)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn get_errors(&self) -> &[LexingError] {
        &self.errors
    }
}
