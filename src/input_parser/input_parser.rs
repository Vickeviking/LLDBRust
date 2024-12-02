use crate::input_parser::command::Command;

// Parses and validates user input for debugger commands
pub struct InputParser {
    // List of valid commands for the debugger
    pub commands: Vec<Command>,
}
