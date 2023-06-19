use std::{cell::RefCell, rc::Rc};

use super::Memory;

pub fn print_as_utf8(memory: Rc<RefCell<Memory>>) {
    let mut result = Vec::new();
    let mem = memory.borrow().memory_view();

    for item in mem {
        result.push(item);
    }

    println!("{}", String::from_utf16(&result).unwrap());
}
