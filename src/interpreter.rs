use crate::domain::{Memory, MemoryError};
use crate::parser::tokenizer::Token;

pub enum InterpreterError {
    UsingBinaryAsUnary,
    MemErr(MemoryError),
}

pub fn interprete(memory: &mut Box<Memory>, tokens: Vec<Token>) -> Result<(), InterpreterError> {
    let mut next_should_be_number = false;
    let mut token_before: Option<Token> = None;

    for token in tokens {
        if next_should_be_number {
            next_should_be_number = false;

            if let Token::Number(value) = token {
                if token_before.is_none() {
                    return Err(InterpreterError::UsingBinaryAsUnary);
                }

                match token_before.unwrap() {
                    Token::Add => {
                        if let Err(value) = memory.add(value) {
                            return Err(InterpreterError::MemErr(value));
                        }
                    }
                    Token::AddModulo => {
                        memory.add_modulo(value);
                    }
                    _ => {
                        return Err(InterpreterError::UsingBinaryAsUnary);
                    }
                };
            }
        }

        match token {
            Token::Number(value) => {
                memory.write(value);
            }
            Token::ShiftLeft => {
                if let Err(value) = memory.shift_left() {
                    return Err(InterpreterError::MemErr(value));
                }
            }
            Token::ShiftRight => {
                if let Err(value) = memory.shift_right() {
                    return Err(InterpreterError::MemErr(value));
                }
            }
            Token::ShiftWordLeft => {
                memory.shift_word_left();
            }
            Token::ShiftWordRight => {
                memory.shift_word_right();
            }
            Token::Add => {
                next_should_be_number = true;
            }
            Token::AddModulo => {
                next_should_be_number = true;
            }
        };

        token_before = Some(token);
    }

    if next_should_be_number {
        return Err(InterpreterError::UsingBinaryAsUnary);
    }

    Ok(())
}
