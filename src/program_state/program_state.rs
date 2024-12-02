// Enumerates the possible statuses of the program
pub enum ProgramStatus {
    // The program is currently running
    Running,
    // The program is currently paused (e.g., at a breakpoint or manually paused)
    Paused,
    // The program has completed execution
    Terminated,
}

pub struct ProgramState {
    // Indicates whether the program is running, paused, or terminated
    pub status: ProgramStatus,
    // The exit code if the program has terminated (None if still running or paused)
    pub exit_code: Option<i32>,
}
