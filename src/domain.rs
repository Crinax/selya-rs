#[derive(Debug)]
pub struct Memory {
    size: u16,
    memory: Vec<u16>,
    position: usize,
    position_as_u16: u16,
}

#[derive(Debug)]
pub enum MemoryError {
    Overflow,
    OutOfRange,
}

#[derive(Debug)]
pub enum ShiftDirection {
    Right,
    Left,
}

impl Memory {
    pub fn new(size: u16) -> Self {
        Memory {
            size,
            memory: vec![0; size.into()],
            position: 0,
            position_as_u16: 0,
        }
    }

    fn update_position(&mut self, new_value: u16) -> u16 {
        self.position_as_u16 = new_value;
        self.position = new_value.into();

        new_value
    }

    pub fn write(&mut self, value: u16) -> u16 {
        self.memory[self.position] = value;

        value
    }

    pub fn add(&mut self, value: u16) -> Result<u16, MemoryError> {
        let current = self.memory[self.position];

        if let Some(result) = current.checked_add(value) {
            self.memory[self.position] = result;
            return Ok(result);
        }

        Err(MemoryError::Overflow)
    }

    pub fn add_modulo(&mut self, value: u16) -> u16 {
        self.memory[self.position] ^= value;

        value
    }

    fn shift(&mut self, value: u16, direction: ShiftDirection) -> Result<u16, MemoryError> {
        match direction {
            ShiftDirection::Left => {
                match self.position_as_u16.checked_add(value) {
                    Some(value) => Ok(self.update_position(value)),
                    None => Err(MemoryError::OutOfRange),
                }
            },
            ShiftDirection::Right => {
                match self.position_as_u16.checked_sub(value) {
                    Some(value) => Ok(self.update_position(value)),
                    None => Err(MemoryError::OutOfRange)
                }
            }
        }
    }

    pub fn shift_right(&mut self) -> Result<u16, MemoryError> {
        self.shift(1, ShiftDirection::Right)
    }

    pub fn shift_left(&mut self) -> Result<u16, MemoryError> {
        self.shift(1, ShiftDirection::Left)
    }

    fn rotate(&mut self, value: usize, direction: ShiftDirection) {
        let guard: usize = self.size.into();

        match direction {
            ShiftDirection::Right => self.memory.rotate_right(value % guard),
            ShiftDirection::Left => self.memory.rotate_left(value % guard),
        }
    }

    pub fn shift_word_left(&mut self) {
        self.rotate(1, ShiftDirection::Left)
    }

    pub fn shift_word_right(&mut self) {
        self.rotate(1, ShiftDirection::Right)
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> u16 {
        self.size
    }

    pub fn memory_view(&self) -> Vec<u16> {
        self.memory.clone()
    }
}

impl IntoIterator for Memory {
    type IntoIter = std::vec::IntoIter<u16>;
    type Item = u16;
    
    fn into_iter(self) -> Self::IntoIter {
        self.memory.into_iter()
    }
}
