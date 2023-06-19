use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::domain::Memory;
use crate::plugin::SelyaPlugin;

pub struct MemoryExecutor {
    plugins: HashMap<String, Box<dyn SelyaPlugin>>,
    memory: Rc<RefCell<Memory>>,
}

impl MemoryExecutor {
    pub fn new(memory: Memory) -> Self {
        MemoryExecutor {
            plugins: HashMap::new(),
            memory: Rc::new(RefCell::new(memory)),
        }
    }

    pub fn register_plugin(&mut self, name: String, plugin: Box<dyn SelyaPlugin>) {
        self.plugins.insert(name, plugin);
    }

    pub fn execute(&self, order: Vec<String>) {
        for name in order {
            self.plugins[&name].execute(self.memory.clone());
        }
    }
}
