use std::io::{self};

pub fn wait_for_user_input() {
    println!("Press Enter to exit...");
    // Sets up new string for user input
    let mut buffer = String::new(); 
    // Waits for user input w/error handler.
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}