#[derive(Debug)]
pub enum Token {
    Number(i128),
    ShiftLeft,
    ShiftRight,
    ShiftWordLeft,
    ShiftWordRight,
    Add,
    AddModulo,
}
