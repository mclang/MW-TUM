//
// Normal console printing
//

// Import crate global constants
use crate::APP_NAME;

// Import functions from SIBLING modules:
use super::debug::print_running_time;


pub fn print_start_message() {
    let header         = format!("::: --> {} <-- :::", APP_NAME.to_uppercase());
    let commas         = vec![':'; header.len()];
    let vbreak: String = commas.into_iter().collect();

    println!("{vbreak}");
    println!("{header}");
    println!("{vbreak}");
}

pub fn print_end_message() {
    print_running_time();
    println!("::: SEE YOU LATER MECHWARRIOR :::");
    println!("");
}
