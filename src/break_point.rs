pub struct Breakpoint {
    // Memory address where the breakpoint is set
    pub address: u64,
    // Line number where the breakpoint is set (optional)
    pub line: u32,
    // Optional condition for the breakpoint to be hit
    pub condition: Option<String>,
}
