pub enum DebugEvent {
    // Represents a variable assignment event.
    Assignment {
        // The name of the variable being assigned
        var_name: String,
        // The new value assigned to the variable
        new_value: String,
        // Tracks the frame in which the assignment happened
        frame_id: Option<u64>,
    },
    // Represents a move operation event for a variable.
    Move {
        // The name of the variable being moved
        var_name: String,
        // Tracks the frame for the move event
        frame_id: Option<u64>,
    },

    // Represents an event where a pointer's value is updated.
    PointerUpdate {
        // The memory address of the pointer being updated
        pointer_address: u64,
        // The new value that the pointer points to
        new_value: String,
        // Tracks the frame for pointer updates
        frame_id: Option<u64>,
    },

    // Represents a function call event.
    FunctionCall {
        // Vector of memory addresses pointing to the parameter variables
        parameters: Vec<u64>,
        // The name of the function being called
        function_name: String,
        // The frame in which the function call happens
        frame_id: Option<u64>,
    },

    // Represents a return call from a function.
    ReturnCall {
        // The address where the function returns to
        return_address: u64,
        // The return value is stored as a memory address in the heap/stack
        return_value: u64,
    },
}
