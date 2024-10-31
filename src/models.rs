#[derive(Debug)]
pub struct UserStory {
    pub description: String,
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug)]
pub struct TestCase {
    pub name: String,
    pub scenarios: Vec<TestScenario>,
}

#[derive(Debug)]
pub struct TestScenario {
    pub scenario_type: String,
    pub given: String,
    pub when: String,
    pub then: String,
}
