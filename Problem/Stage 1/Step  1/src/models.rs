use std::process::exit;

use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: Option<String>,
    pub stories: Vec<Story>,
    pub status: Status,
}

impl Epic {
    fn new() -> Self {
        let mut reader = Reedline::create();

        let name = get_name(&reader);

        let description = get_description(&reader);

        return Epic {
            name,
            description,
            stories: vec![],
            status: Status::Open,
        };
    }

    fn add_story(&mut self) {
        let story = Story::new();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Story {
    name: String,
    description: Option<String>,
    status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        return Story {
            name,
            description,
            status: Status::Open,
        };
    }

    pub fn update_status(&mut self, status: Status) {
        let status = Self::get_status_char(&reader);
        self.Status = status;
    }
}

pub struct DBState {
    last
}
