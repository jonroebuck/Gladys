// parser.rs

use serde_json::Error;
use Gladys::models::UserStory;

pub fn parse_input(input: &str) -> Result<UserStory, Error> {
    serde_json::from_str(input)
}

// Tests for the parser
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_input_valid_json() {
        let json_input = r#"
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

        let result = parse_input(json_input);
        assert!(result.is_ok(), "Expected Ok but got an error: {:?}", result.err());
        
        let user_story = result.unwrap();
        assert_eq!(user_story.description, "As a bank customer I want to register for a rewards program");
        assert_eq!(user_story.test_cases.len(), 1);
        
        let test_case = &user_story.test_cases[0];
        assert_eq!(test_case.case, "Successful Registration");
        assert_eq!(test_case.scenarios.len(), 1);
        
        let scenario = &test_case.scenarios[0];
        assert_eq!(scenario.scenario_type, "Happy Flow");
        assert_eq!(scenario.given, "I am a registered bank customer");
        assert_eq!(scenario.when, "I enter a valid membership ID");
        assert_eq!(scenario.then, "I am successfully registered");
    }

    #[test]
    fn test_parse_input_invalid_json() {
        let invalid_json = r#"
            {
                "story": "As a bank customer I want to register for a rewards program",
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
                ]  // Missing closing brace
        "#;

        let result = parse_input(invalid_json);
        assert!(result.is_err(), "Expected an error but got Ok: {:?}", result);
    }
}
