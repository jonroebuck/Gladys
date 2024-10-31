pub struct UserStory {
    pub story: String,
    pub test_cases: Vec<TestCase>,
}

pub struct TestCase {
    pub case: String,
    pub scenarios: Vec<TestScenario>,
}

pub struct TestScenario {
    pub scenario_type: String,
    pub given: String,
    pub when: String,
    pub then: String,
}

pub fn parse_input(input: &str) -> UserStory {
    // Implement parsing logic here
    // For simplicity, this is just a placeholder
    UserStory {
        story: String::from("As a bank customer I want to register for a rewards program"),
        test_cases: vec![TestCase {
            case: String::from("Successful Registration"),
            scenarios: vec![TestScenario {
                scenario_type: String::from("Happy Flow"),
                given: String::from("I am a registered bank customer"),
                when: String::from("I enter a valid membership ID"),
                then: String::from("I am successfully registered"),
            }],
        }],
    }
}

