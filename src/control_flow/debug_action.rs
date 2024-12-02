// Enumerates the possible actions to control the debugger's flow
pub enum DebugAction {
    // Step to the next line of code
    Step,
    // Continue execution until the next breakpoint or program termination
    Continue,
    // Pause the execution at the current state
    Pause,
    // Step into a function call
    StepIn,
    // Step out of the current function
    StepOut,
}
