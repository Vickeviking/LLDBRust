use std::collections::HashMap;

pub struct Frame {
    pub function_name: String,
    //maps local_variables to memaddresses
    pub local_variables: HashMap<String, u64>,
    pub frame_id: u64,
}
