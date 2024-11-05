use crate::models::UserStory;
use std::error::Error;

pub trait Parser {
    fn parse_input(&self, input: &str) -> Result<UserStory, Box<dyn Error>>;
}