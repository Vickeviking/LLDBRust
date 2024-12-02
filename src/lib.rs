pub mod control_flow;
pub mod debugger_state;
pub mod input_parser;
pub mod lldb_interface;
pub mod memory;
pub mod output_handler;
pub mod program_state;

use lldb::{LaunchFlags, SBDebugger, SBLaunchInfo, SBProcess, SBTarget};

pub fn run_debugger() {
    // Initialize LLDB
    SBDebugger::initialize();

    // Create a debugger instance
    let dbg = SBDebugger::create(false);

    // Create a target (e.g., for a program)
    let target = dbg
        .create_target_simple("test_projects/target/debug/test_programs")
        .expect("Failed to create target");

    println!("Target loaded.");

    // Prepare the launch info
    let launch_info = SBLaunchInfo::new();
    launch_info.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);

    // Launch the program using the SBTarget::launch() method
    let process = target
        .launch(launch_info)
        .expect("Failed to launch process");

    println!("Process started.");

    // Monitor process state
    while process.is_alive() {
        if process.is_stopped() {
            println!("Process stopped. Press Enter to continue.");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            process
                .continue_execution()
                .expect("Failed to continue process");
        } else if process.is_running() {
            println!("Process is running...");
        }
    }

    println!("Process is no longer alive.");

    // Terminate the debugger
    SBDebugger::terminate();
}
