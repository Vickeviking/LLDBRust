use std::collections::HashMap;

pub struct Object {
    pub memory_address: u64,
    pub class_name: String,
    // Field name -> Memory address
    pub fields: HashMap<String, u64>,
}
//implements memObject from mem
