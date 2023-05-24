//
// Module for all console related functions
//

// Private submodules:
mod debug;
mod output;

// Public interface 0f the module.
// Export ONLY those functions that are used in the main program or in other modules:
pub use output::{print_start_message, print_end_message};
