use std::{cell::RefMut, borrow::{Borrow, BorrowMut}};

use super::Memory;

pub fn print_as_utf8(memory: RefMut<Memory>) {
    let mut result = Vec::new();
    let mut mem = memory.borrow_mut();

    for element in mem.into_iter() {
        result.push(element);
    }

    println!("{}", String::from_utf16(&result).unwrap());
}