#[derive(Debug)]
pub enum Token {
    Number(u16),
    ShiftLeft,
    ShiftRight,
    ShiftWordLeft,
    ShiftWordRight,
    Add,
    AddModulo,
}

pub trait UnwrapToken {
    fn unwrap(&self) -> u16;
}

impl UnwrapToken for Token {
    fn unwrap(&self) -> u16 {
        if let Token::Number(value) = self {
            return *value;
        }

        panic!("Cannot unwrap token")
    }
}
