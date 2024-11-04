use domain::models::UserStory;
use domain::parser::parse_input; 

//use crate domain::{parser::{parse_input, UserStory}}; // Import necessary items from the parser module
//use crate domain::models;

fn main() {
    // Example usage of parse_input
    let input = "some input";
    match domain::parser::parse_input(input) {
        Ok(result) => {
            // Handle success (e.g., print the result)
            println!("Parsed result: {:?}", result);
        }
        Err(e) => {
            // Handle error (e.g., print the error)
            eprintln!("Error: {:?}", e);
        }
    }
}
