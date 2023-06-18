use std::{cell::RefCell, rc::Rc};

use super::Memory;

pub fn print_as_utf8(memory: Rc<RefCell<Memory>>) {
    let mut result = Vec::new();
    
    memory
        .clone()
        .borrow_mut()
        .into_iter()
        .map(|| {
            
        });

    println!("{}", String::from_utf16(&result).unwrap());
}
