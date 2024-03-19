use std::io::{self};

mod commands;
use commands::*;

fn main() {
    // Get website URL from user input
    let mut website_url = String::new();
    println!("Enter the SHA256SUMS file URL:");
    io::stdin().read_line(&mut website_url).expect("Failed to read line");
    let website_url = website_url.trim();

    // Fetch the website hashes
    let hashes = hashwork::fetch_website_hashes(website_url);

    // Display available hashes
    if let Some(hashes) = &hashes {
        println!("Available hashes:");
        for (index, (hash, file_name)) in hashes.iter().enumerate() {
            println!("{}. {} - {}", index + 1, file_name, hash);
        }

        // Get user choice
        println!("Enter the number of the hash you want to use:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                return;
            }
        };

        // Check if the choice is within bounds
        if choice <= 0 || choice > hashes.len() {
            eprintln!("Invalid choice. Please enter a valid number.");
            return;
        }

        let (selected_hash, selected_file) = &hashes[choice - 1];

        // Get file path from user input
        let mut file_path = String::new();
        println!("Enter the file path for {}:", selected_file);
        io::stdin().read_line(&mut file_path).expect("Failed to read line");
        let file_path = file_path.trim();

        // Calculate file hash
        match hashwork::calculate_file_hash(file_path) {
            Ok(file_hash) => {
                // Compare the hashes
                if selected_hash == &file_hash {
                    println!("Hashes match! The file is valid.");
                } else {
                    println!("Hashes do not match. The file may be corrupted or tampered with.");
                }
            }
            Err(e) => eprintln!("Error calculating file hash: {}", e),
        }
    } else {
        println!("Failed to fetch website hashes. Please check the URL and try again.");
    }

exit::wait_for_user_input();
}