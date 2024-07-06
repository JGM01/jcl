use std::{iter::Peekable, str::Chars};

use super::token::{Token, TokenType, Value, Position, Operator, Keyword};

#[derive(Debug, Clone)]
struct LexingError {
    message: String,
    position: Position,
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

    fn next_token(&mut self) -> Option<Token> {

        // Define the token by checking the current character, and then handle.
        let token: Token = match self.current_char {

            // Pattern match for different tokens.
            Some(c) => match c {
                'a'..='z' | 'A'..='Z' | '_' => self.handle_symbol(),
                '0'..='9' => self.handle_number(),
                '"' => self.handle_string(),
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '&' | '|' => self.handle_operator(),
                ';' | ',' | '(' | ')' | '{' | '}' | '[' | ']' => self.handle_punctuator(),

                // If there is no handler for the character, push an error with some info to the error list.
                // Also, make sure the lexer still continues it's advancement.
                _ => {
                    let error_position = self.position.clone();
                    self.errors.push(LexingError {
                        message: format!("Unexpected character: {}", c),
                        position: error_position,
                    });
                    self.advance();
                    return self.next_token()
                }
            },
            None => return None,
        };
        Some(token)
    }

    fn handle_operator(&mut self) -> Token {
        todo!()
    }

    fn handle_punctuator(&mut self) -> Token {
        todo!()
    }

    fn handle_symbol(&mut self) -> Token {
        todo!()
    }

    fn handle_number(&mut self) -> Token {
        todo!()
    }

    fn handle_string(&mut self) -> Token {
        todo!()
    }
}
