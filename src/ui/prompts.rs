use crate::{
    models::{Epic, Story, Status}, 
    io_utils::get_user_input
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_name: Box<dyn Fn() -> String>,
    pub update_description: Box<dyn Fn() -> String>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
    pub reset_database: Box<dyn Fn() -> bool>,
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_name: Box::new(update_name_prompt),
            update_description: Box::new(update_description_prompt),
            update_status: Box::new(update_status_prompt),
            reset_database: Box::new(reset_database_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("-------------------------------------------------------------------------");
    println!("Epic Name: \n");

    let epic_name = get_user_input().trim().to_owned();

    println!("-------------------------------------------------------------------------");
    println!("Epic Description: \n");

    let epic_desc = get_user_input().trim().to_owned();

    let epic = Epic::new(epic_name, epic_desc);

    epic
}

fn update_name_prompt() -> String {
    println!("-------------------------------------------------------------------------");
    println!("New Name: \n");

    let new_name = get_user_input().trim().to_owned();

    new_name
}

fn update_description_prompt() -> String {
    println!("-------------------------------------------------------------------------");
    println!("New Description: \n");

    let new_description = get_user_input().trim().to_owned();

    new_description
}

fn create_story_prompt() -> Story {
    println!("-------------------------------------------------------------------------");
    println!("Story Name: \n");

    let story_name = get_user_input().trim().to_owned();

    println!("-------------------------------------------------------------------------");
    println!("Story Description: \n");

    let story_desc = get_user_input().trim().to_owned();

    let story = Story::new(story_name, story_desc);

    story
}

fn delete_epic_prompt() -> bool {
    println!("-------------------------------------------------------------------------");
    println!("Are you sure you want to delete this epic?");
    println!("All stories in this epic will also be deleted [Y/n]:");
    
    let confirmation = get_user_input();

    if confirmation.trim().eq("Y") {
        return true
    }
    false
}

fn delete_story_prompt() -> bool {
    println!("-------------------------------------------------------------------------");
    println!("Are you sure you want to delete this story? [Y/n]:");

    let confirmation = get_user_input();

    if confirmation.trim().eq("Y") {
        return true
    }
    false
}

fn update_status_prompt() -> Option<Status> {
    println!("-------------------------------------------------------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");

    let status = get_user_input().trim().parse::<u8>();

    if let Ok(status) = status {
        match status {
            1 => Some(Status::Open),
            2 => Some(Status::InProgress),
            3 => Some(Status::Resolved),
            4 => Some(Status::Closed),
            _ => None
        }
    } else {
        None
    }
}

fn reset_database_prompt() -> bool {
    println!("-------------------------------------------------------------------------");
    println!("Are you sure you want to reset the database? [Y/n]:");

    let confirmation = get_user_input();

    if confirmation.trim().eq("Y") {
        return true
    }
    false
}