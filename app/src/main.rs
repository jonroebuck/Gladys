use domain::models::UserStory;
use app::json_parser::JsonParser; // Import JsonParser from the app crate
use domain::parser::Parser; // Import the Parser trait

fn main() {
    // Example usage of parse_input
    let input = r#"
        {
            "description": "As a bank customer I want to register for a rewards program",
            "test_cases": [
                {
                    "case": "Successful Registration",
                    "scenarios": [
                        {
                            "scenario_type": "Happy Flow",
                            "given": "I am a registered bank customer",
                            "when": "I enter a valid membership ID",
                            "then": "I am successfully registered"
                        }
                    ]
                }
            ]
        }
    "#;

    let parser = JsonParser; // Create an instance of JsonParser
    match parser.parse_input(input) {
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
