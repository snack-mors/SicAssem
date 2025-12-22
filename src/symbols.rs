use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Symbol {
    pub address: i32,
    pub source_line: i32,
}
pub struct SymbolTable {
    // Note how we don't declare map as pub, this makes it a private field.
    map: HashMap<String, Symbol>,
}
impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            map: HashMap::new(),
        }
    }
    // Note how in the insert we have to declare that a pointer is &mut = MUTABLE.
    pub fn insert(&mut self, name: String, address: i32, source_line: i32) -> Result<(), String> {
        if name.len() > 6 {
            return Err("Symbol name cannot be more than 6 characters".to_string());
        }
        if self.map.contains_key(&name) {
            return Err(format!("Duplicate symbol name: {}", name));
        }
        let sym = Symbol {
            address,
            source_line,
        };
        self.map.insert(name, sym);
        Ok(())
    }
    pub fn print_symbols(&self) {
        println!("Symbol Table Content:");
        println!("---------------------");
        // Collect all entries into a vector to sort them.
        // Note that we borrow READABLE references.
        let mut entries: Vec<(&String, &Symbol)> = self.map.iter().collect();
        // Sort by alphabetical name (key)
        entries.sort_by(|a, b| a.1.address.cmp(&b.1.address));
        // For each entry in entries, note how the loop itself destructures the touple.
        for (name, sym) in entries {
            println!("{:<8} | {:04X}", name, sym.address);
        }

    }


    pub fn get_address(&self, name: &str) -> Option<i32> {
        self.map.get(name).map(|sym| sym.address)
    }
}