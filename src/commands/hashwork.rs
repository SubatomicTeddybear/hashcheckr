use std::fs::File;
use std::io::{self,BufReader,BufRead};
use reqwest;
use sha2::{Digest, Sha256};

pub fn calculate_file_hash(file_path: &str) -> Result<String, io::Error> {
    // Get file path and open
    let file = File::open(file_path)?; 
    // Set up new Sha256 for later use
    let mut hasher = Sha256::new(); 
    // Set up buffer with 1024 bytes capacity, referencing file.
    let mut reader = BufReader::with_capacity(1024,file);

    loop {
        // Attempt to fill the buffer with data from the reader, returning a Result containing the buffer.
        let buffer = reader.fill_buf()?;
        // Get the number of bytes read into the buffer.
        let bytes_read = buffer.len();
        // Check if no bytes were read, indicating the end of the data stream, and if so, break out of the loop.
        if bytes_read == 0 {break; }

        // Update the hasher with the data in the buffer, up to the number of bytes read.
        hasher.update(&buffer[..bytes_read]);
        // Consume the bytes read from the reader, advancing the internal cursor.
        reader.consume(bytes_read);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn fetch_website_hashes(website_url: &str) -> Option<Vec<(String, String)>> {
    // Match the result of sending a blocking request to the specified website URL using reqwest.
    match reqwest::blocking::get(website_url) {
        // If the request is successful, unwrap the response.
        Ok(response) => {
            // Check if the response status indicates success.
            if response.status().is_success() {
                // Attempt to retrieve the text content of the response, returning None if unsuccessful.
                let content = response.text().ok()?;

                // Extract hashes and file names from each line of the content.
                let hashes: Vec<(String, String)> = content
                    // Iterate over lines of the content.
                    .lines()
                    // Filter out lines that cannot be processed.
                    .filter_map(|line| {
                        // Split the line into parts based on whitespace.
                        let mut parts = line.split_whitespace(); // Splits line by whitespace
                        // Retrieve the hash, which is the first part of the line.
                        let hash = parts.next()?;
                        // Retrieve the file name, which is the second part of the line.
                        let file_name = parts.next()?;
                        // Return a tuple containing the hash and file name as strings, converting them from &str to String.
                        Some((hash.to_string(), file_name.to_string()))
                    })
                    // Collect the tuples into a vector.
                    .collect();

                // Return Some containing the vector of hashes and file names.
                return Some(hashes);
            }
        }
        // If an error occurs during the request, print an error message to stderr.
        Err(e) => eprintln!("Error fetching website hashes: {}", e),
    }
    // Return None if the request was unsuccessful or encountered an error.
    None
}

