#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "<User Story>As a bank customer I want to register for a rewards program\
                     <Test Case>Successful Registration\
                     <Test Scenario Type>Happy Flow</Test Scenario Type>\
                     <Given>I am a registered bank customer</Given>\
                     <When>I enter a valid membership ID</When>\
                     <Then>I am successfully registered</Then>";
        
        let parsed = parse_input(input);
        
        // Assertions to validate the parsing
        assert_eq!(parsed.story, "As a bank customer I want to register for a rewards program");
        assert_eq!(parsed.test_cases.len(), 1);
        
        let test_case = &parsed.test_cases[0];
        assert_eq!(test_case.case, "Successful Registration");
        assert_eq!(test_case.scenarios.len(), 1);
        
        let scenario = &test_case.scenarios[0];
        assert_eq!(scenario.scenario_type, "Happy Flow");
        assert_eq!(scenario.given, "I am a registered bank customer");
        assert_eq!(scenario.when, "I enter a valid membership ID");
        assert_eq!(scenario.then, "I am successfully registered");
    }
}
