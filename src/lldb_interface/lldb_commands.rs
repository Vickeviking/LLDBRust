// Enum for various commands that can be sent to LLDB
pub enum LLDBCommand {
    // Attach to a running process with the given PID
    AttachProcess(u64),
    // Set a breakpoint at the given address
    SetBreakpoint(u64),
    // Continue program execution
    ContinueExecution,
    // Step over the next line in the program
    Step,
    // Step into the next function call
    StepInto,
    // Step out of the current function
    StepOut,
    // Get the current state of the program (running, paused, etc.)
    GetProgramState,
    // Quit the LLDB session
    Quit,
}
