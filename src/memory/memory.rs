use crate::memory::variable_info::VariableInfo;
use std::collections::HashMap;

pub trait MemObject {
    // Returns a string representation of the object
    fn display(&self) -> String;
    // Indicates if the object is stack-allocated
    fn is_stack_allocated(&self) -> bool;
    fn get_frame(&self) -> Option<u64>;
}

pub struct Mem {
    pub memory: HashMap<u64, Box<dyn MemObject>>, // All memory addresses -> MemObjects (stack + heap)
}

pub struct Memory {
    // Maps variable names to memory addresses
    pub name_to_address: HashMap<String, u64>,
    // Stores information about variables at specific memory addresses
    pub variables: HashMap<u64, VariableInfo>,
    // Tracks the actual memory (stack/heap)
    pub mem: Mem,
}
