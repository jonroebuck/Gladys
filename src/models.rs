use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStory {
    pub description: String,
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub case: String,
    pub scenarios: Vec<TestScenario>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestScenario {
    pub scenario_type: String,
    pub given: String,
    pub when: String,
    pub then: String,
}
