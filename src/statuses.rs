use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Iniciativas {
    pub inis: HashMap<String, u32>,
}

impl Iniciativas {
    pub fn new() -> Self {
        Self {
            inis: HashMap::new(),
        }
    }
}

pub static INICIA_GERAL: Lazy<Mutex<Iniciativas>> = Lazy::new(|| Mutex::new(Iniciativas::new()));
