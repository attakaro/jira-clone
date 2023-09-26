use std::collections::HashMap;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicName {epic_id: u32},
    UpdateEpicDescription {epic_id: u32},
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryName {story_id: u32},
    UpdateStoryDescription {story_id: u32},
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    ReadEpicDescription { epic_id: u32 },
    ReadStoryDescription { story_id: u32 },
    ResetDatabase,
    Exit,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Status { 
    Open, 
    InProgress, 
    Resolved,
    Closed 
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => f.write_str("OPEN")?,
            Self::InProgress => f.write_str("IN PROGRESS")?,
            Self::Resolved => f.write_str("RESOLVED")?,
            Self::Closed => f.write_str("CLOSED")?
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Epic { 
    pub name: String, 
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {   
            name, 
            description, 
            status: Status::Open, 
            stories: Vec::new() 
        } 
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Story {
    pub name: String, 
    pub description: String,
    pub status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {  
            name, 
            description,
            status: Status::Open 
        } 
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DBState {
    pub last_item_id: u32,  
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}