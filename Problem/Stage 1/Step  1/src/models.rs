use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
    pub stories: Vec<u64>,
    pub status: Status,
}

impl Epic {
    fn new(name: String, description: String) -> Self {
        return Epic {
            name,
            description,
            stories: vec![],
            status: Status::Open,
        };
    }

    fn add_story(&mut self, story_id: u64) {
        stories.push(story_id);
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

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct DBState {
    last_item_id: u64,
    epics: HashMap<u64, Epic>,
    stories: HashMap<u64, Story>,
}

impl DBState {
    fn new() -> Self {
        return DBState {
            last_item_id: 0,
            epics: HashMap::new(),
            stories: HashMap::new()
        }
    }

    fn add_epic(&mut self, epic: Epic) {
        let new_id = self.last_item_id + 1;
        self.epics.insert(new_id, epic);
        self.last_item_id += 1;
    }

    fn add_story(&mut self, story: Story) {
        let new_id = self.last_item_id + 1;
        self.stories.insert(new_id, story);
        self.last_item_id += 1;
    } 
}