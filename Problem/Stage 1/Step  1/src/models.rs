use std::process::exit;

use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use serde::{Deserialize, Serialize};

fn get_name(reader: &Reedline) -> String {
    let name_prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic(
            "What is the name of your new Epic? (Once set, cannot be changed!)".to_owned(),
        ),
        DefaultPromptSegment::Empty,
    );

    let mut name: String;
    loop {
        let signal = reader.read_line(&name_prompt);
        match signal {
            Ok(Signal::Success(buffer)) => {
                name = buffer;
                break;
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            Err(x) => {
                println!("Error: {x:?}");
            }
        }
    }

    return name;
}

fn get_description(reader: &Reedline) -> Option<String> {
    let description_prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic(
            "Write an description of the project, or leave empty to skip.".to_owned(),
        ),
        DefaultPromptSegment::Empty,
    );
    let mut description: Option<String>;
    loop {
        let signal = reader.read_line(&description_prompt);
        match signal {
            Ok(Signal::Success(buffer)) => {
                if buffer == String::new() {
                    description = None;
                    break;
                }
                description = Some(buffer);
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            Err(x) => {
                println!("Error: {x:?}");
            }
        }
    }

    return description;
}

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
    pub fn new() -> Self {
        let mut reader = Reedline::create();

        let name = get_name(&reader);

        let description = get_description(&reader);

        return Story {
            name,
            description,
            status: Status::Open,
        };
    }

    pub fn update_status(&mut self) {
        loop {
            let mut reader = Reedline::create();

            //set looping readline promt that stops when appropriate input is given
            loop {
                let status = Self::get_status_char(&reader);
                self.Status = match status {
                    'p' | 'P' => Status::InProgress,
                    'r' | 'R' => Status::Resolved,
                    'c' | 'C' => Status::Closed,
                    _ => {
                        println!("Invalid response, please try again");
                        continue;
                    }
                };
            }
        }
    }

    fn get_status_char(reader: &Reedline) -> char {
        let status_prompt = DefaultPrompt::new(DefaultPromptSegment::Basic("Please enter the appropriate status of this story below: In[P]rogress/[R]esolved/[C]losed\n".to_owned()), DefaultPromptSegment::Empty);
        loop {
            let signal = reader.read_line(&status_prompt);
            match signal {
                Ok(Signal::Success(buffer)) => {
                    if String::len(&buffer) == 1 {
                        let (unsign, ch) = buffer.char_indices().next().unwrap();
                        return ch;
                    } else {
                        println!("Too many characters. Please try again.");
                        continue;
                    }
                }
                Ok(Signal::CtrlC | Signal::CtrlD) => {
                    println!("\nAborted!");
                    exit(0);
                }
                Err(x) => {
                    println!("Error: {x:?}");
                    continue;
                }
            }
        }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
}
