use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub struct LoopingStatus{
    pub loop_track: bool,
}

pub struct Iniciativas{
    pub inis: HashMap<String,u32>,
}

impl LoopingStatus {
    pub fn new() -> Self{
        Self {loop_track:false}
    }
}

impl Iniciativas{
    pub fn new() -> Self{ Self {inis:HashMap::new()}}
}

pub static SHARED_LOOP : Lazy<Mutex<LoopingStatus>> = Lazy::new(||
    Mutex::new(LoopingStatus::new()));

pub static INICIA_GERAL : Lazy<Mutex<Iniciativas>> = Lazy::new(|| 
    Mutex::new(Iniciativas::new()));

