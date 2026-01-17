use std::collections::HashMap;

use crate::obj::ValueObj;

pub struct Scope<'a> {
    table: HashMap<&'a str, ValueObj>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Scope {
            table: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &'a str, value: ValueObj) -> bool {
        if self.table.contains_key(name) {
            false
        } else {
            self.table.insert(name, value);
            true
        }
    }

    pub fn get(&self, name: &str) -> Option<&ValueObj> {
        self.table.get(name)
    }

    pub fn set(&mut self, name: &'a str, value: ValueObj) -> bool {
        if self.table.contains_key(name) {
            self.table.insert(name, value);
            true
        } else {
            false
        }
    }
}
