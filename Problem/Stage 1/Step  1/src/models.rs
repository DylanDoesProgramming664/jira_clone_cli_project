pub enum Status {
    Initialized,
    Pending,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub stories: Vec<Story>,
    pub status: Status,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        return Epic {
            name,
            description,
            stories: vec![],
            status: Status::Initialized,
        };
    }

    pub fn add_story(&mut self) {
        let story = Story::new();
    }
}

pub struct Story {
    name: String,
    description: String,
    status: Status,
}

impl Story {
    pub fn new() -> Self {
        let name = todo!();
        let description = todo!();
        return Story {
            name,
            description,
            status: Status::Initialized,
        };
    }

    pub fn update_status(&mut self) {
        loop {
            println!("Please enter the appropriate status of this story below: ");
            //set looping readline promt that stops when appropriate input is given
            let input = todo!();
            self.Status = match status {
                'p' | 'P' => Status::Pending,
                'c' | 'C' => Status::Complete,
                'f' | 'F' => Status::Failed,
                _ => {
                    println!("Invalid response, please try again");
                    continue;
                }
            };
        }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
}
