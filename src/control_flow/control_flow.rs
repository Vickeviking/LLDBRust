use crate::control_flow::debug_action::DebugAction;
// Represents the control flow actions available in the debugger
pub struct ControlFlow {
    // Indicates whether the debugger is currently paused or running
    pub is_paused: bool,
    // The last action taken by the debugger (e.g., Step, Continue, Pause)
    pub last_action: DebugAction,
}
