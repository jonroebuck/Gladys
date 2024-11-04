use serde_json::Error;

use crate::models::UserStory;

pub fn parse_input(input: &str) -> Result<UserStory, Error> {
    serde_json::from_str(input)
}
