use crate::lldb_interface::lldb_commands::LLDBCommand;
use std::process::Child;
// Represents an interface to communicate with LLDB
pub struct LLDBInterface {
    // Represents the running LLDB process, if any
    pub process: Option<Child>,
    // Queue of LLDB commands that are to be executed
    pub command_queue: Vec<LLDBCommand>,
}
