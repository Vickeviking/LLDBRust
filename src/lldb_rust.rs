pub struct LLDBRust {
    // Holds the debugger's state (call stack, events, breakpoints)
    pub state: DebuggerState,  
     // Manages memory, variables, and addresses
    pub memory: Memory,
    // Represents an interface to communicate with LLDB
    pub lldb_interface: LLDBInterface
    // Tracks the current line of execution in the program
    pub current_line: CurrentLine, 
    // List of active breakpoints set in the program
    pub breakpoints: Vec<Breakpoint>,
    // Keeps track of the current debug event
    pub current_event: Option<DebugEvent>,
    // Controls the flow of execution (e.g., stepping, continuing, pausing)
    pub control_flow: ControlFlow,
    // Keeps track of the program state (running, paused, etc.)
    pub program_state: ProgramState,

}
