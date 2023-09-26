use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() -> Result<(), anyhow::Error> {
    
    let file_path = "./data/db.json".to_owned();
    let db = Rc::new(JiraDatabase::new(file_path));
    let mut navigator = Navigator::new(db.clone());

    loop {
        clearscreen::clear()?;

        match Navigator::get_current_page(&navigator) {

            Some(current_page) => {

                if let Err(error) = current_page.draw_page() {
                    println!("Error rendering page: {}\nPress any key to continue...", error);
                    wait_for_key_press();
                };
                
                let input = get_user_input().trim().to_owned();

                match current_page.handle_input(&input) {

                    Ok(action) => { 
                        if let Some(action) = action {
                            if let Err(err) = navigator.handle_action(action) {
                                println!("Error handling processing user input: {}\nPress any key to continue...", err);
                                wait_for_key_press();
                            }
                        } 
                    }

                    Err(err) => {
                        println!("Error getting user input: {}\nPress any key to continue...", err);
                        wait_for_key_press();
                    }
                }
            }

            None => break
        }
    }

    Ok(())
}