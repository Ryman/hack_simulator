use std::collections::HashMap;

pub struct SymbolTable<'a>(HashMap<&'a str, u16>);

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        SymbolTable(HashMap::new())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn get_address(&self, key: &str) -> u16 {
        self.0[key]
    }

    pub fn add_entry(&mut self, key: &'a str, value: u16) {
        self.0.insert(key, value);
    }
}
