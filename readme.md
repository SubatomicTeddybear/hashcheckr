# ISO Hash Checker

This Rust program allows you to fetch hashes of files from a given website and compare them with locally calculated hashes. It utilizes the SHA256 algorithm for hash calculation and interacts with the user via the command line interface. The formatting is currently built for Ubuntu.
## Build Instructions

To build and run the program, follow these steps:

1. Ensure you have Rust installed on your system. If not, you can install it from Rust's official website.

2. Clone this repository to your local machine or download the code files.

3. Navigate to the directory containing the code files in your terminal.

4. Run the following command to build and run the program in your terminal:

       cargo run

5. Follow the prompts to enter the URL of the SHA256SUMS file and choose a file to verify.

6. After the program finishes execution, press Enter to exit.

## Functionality

The program consists of several modules and functions:
### main.rs

    The main entry point of the program.
    Prompts the user to input the URL of the SHA256SUMS file.
    Fetches the hashes from the specified URL.
    Allows the user to select a file and verify its integrity by comparing hashes.

### commands/mod.rs

    Module file to organize different functionalities of the program.

### commands/hashwork.rs

    Contains functions related to hashing and fetching website hashes.
    calculate_file_hash: Calculates the SHA256 hash of a given file.
    fetch_website_hashes: Fetches hashes of files from a given website URL.

### commands/exit.rs

    Contains a function to wait for user input before exiting the program.

## Contributing

Feel free to contribute to the project by submitting bug reports, feature requests, or pull requests. Your contributions are greatly appreciated!
## License

This project is licensed under the GNU GENERAL PUBLIC LICENSE. See the LICENSE file for details.
