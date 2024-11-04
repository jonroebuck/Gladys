mod parser;
mod models;

fn main() {
    // Example usage of parse_input
    let input = "some input";
    match parser::parse_input(input) {
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
