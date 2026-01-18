use std::collections::HashMap;

use crate::obj::Object;

pub struct Scope {
    table: HashMap<String, Object>,
}

impl Scope {
    pub fn global() -> Self {
        let mut scope = Scope {
            table: HashMap::new(),
        };
        // Populate global scope with built-in functions and variables here if needed
        scope
    }

    pub fn new() -> Self {
        Scope {
            table: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: String, value: Object) -> bool {
        if self.table.contains_key(&name) {
            false
        } else {
            self.table.insert(name, value);
            true
        }
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        self.table.get(name)
    }

    pub fn set(&mut self, name: String, value: Object) -> bool {
        if self.table.contains_key(&name) {
            self.table.insert(name, value);
            true
        } else {
            false
        }
    }
}
