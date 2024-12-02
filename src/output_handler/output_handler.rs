// Controls verbosity levels for the output
pub enum VerbosityLevel {
    Minimal,
    Default,
    Verbose,
}

// Defines customizable format options for the debugger's output
pub struct FormatOptions {
    // Specifies the level of detail (e.g., verbose, minimal)
    pub verbosity: VerbosityLevel,

    // Whether to pretty-print complex structures like enums or structs
    pub pretty_print: bool,
}

// Handles user-facing outputs, such as formatted prints of debugger states and variables
pub struct OutputHandler {
    // Formatting options for displaying variables, memory, and call stacks
    pub format_options: FormatOptions,
}
