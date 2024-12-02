use crate::debugger_state::debug_event::DebugEvent;
use crate::memory::frame::Frame;
use std::collections::HashSet;

pub struct DebuggerState {
    // Track the current debug event
    pub current_debug_event: Option<DebugEvent>,
    // Stack of function call frames
    pub call_stack: Vec<Frame>,
    // History of debug events (step, breakpoints, etc.)
    pub history: Vec<DebugEvent>,
    // Store breakpoints (addresses or line numbers)
    pub breakpoints: HashSet<u64>,
}
