use std::sync::Mutex;
use once_cell::sync::Lazy;

pub struct LoopingStatus{
    pub loop_track: bool,
}

impl LoopingStatus {
    pub fn new() -> Self{
        Self {loop_track:false}
    }
}

pub static SHARED_LOOP : Lazy<Mutex<LoopingStatus>> = Lazy::new(||
    Mutex::new(LoopingStatus::new()));
