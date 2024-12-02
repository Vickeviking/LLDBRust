// Represents an argument for a command (e.g., break <line_number>)
pub struct Argument {
    pub name: String,
    pub optional: bool,
}
// Represents an individual debugger command (e.g., "step", "continue")
pub struct Command {
    pub name: String,             // The name of the command
    pub description: String,      // A brief description of the command
    pub arguments: Vec<Argument>, // List of supported arguments
}
