use crate::helper::{self, handle_binary_input};
use rust_search::SearchBuilder;
use std::{
    fs::{remove_dir_all, remove_file},
    io::{prelude::Write, stdout},
    process::Command,
};

pub fn handle_config_files(application_name: &str, program_name: &str, executable_name: &str) {
    println!("Searching for configuration directories with names \"{application_name}\" or \"{program_name}\" or \"{executable_name}\"");
    let mut config_directory: Vec<String> = SearchBuilder::default()
        .location("~/.config")
        .search_input(application_name)
        .depth(1)
        .build()
        .collect();

    if config_directory.len() != 0 {
        println!("Found config directory: {:?}", config_directory);
    }
    config_directory = SearchBuilder::default()
        .location("~/.config")
        .search_input(program_name)
        .depth(1)
        .build()
        .collect();
    if config_directory.len() != 0 {
        println!("Found config directory: {:?}", config_directory);
    }
    config_directory = SearchBuilder::default()
        .location("~/.config")
        .search_input(executable_name)
        .depth(1)
        .build()
        .collect();
    if config_directory.len() == 0 {
        println!("No config directory found.");
        return;
    }
    println!("Found config directory: {}", config_directory[0]);
    print!("Delete this directory? (yes, no): ");
    stdout().flush().unwrap();
    match handle_binary_input() {
        true => {
            remove_dir_all(config_directory[0].to_owned())
                .expect("Failed to delete config directory!");
            println!("Deleted config directory successfully.");
        }
        false => {
            return;
        }
    }
}

pub fn handle_application_files(app_name: &str) {
    println!("Searching for application files...");
    let application_files: Vec<String> = SearchBuilder::default()
        .more_locations(vec!["/usr", "/var"])
        .search_input(app_name)
        .build()
        .collect();

    if application_files.len() == 0 {
        println!("No application files found.");
        return;
    }
    println!("Found application files- ");
    for file_name in &application_files {
        println!("{file_name}");
    }
    print!("\nConfirm deletion of related files? (yes, no): ");
    stdout().flush().unwrap();
    match helper::handle_binary_input() {
        true => {
            println!("Deleting application files...");
            for file_name in application_files {
                match remove_file(&file_name) {
                    Ok(_) => {}
                    Err(_) => match remove_dir_all(file_name) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Couldn't delete due to error: {e}");
                        }
                    },
                }

                print!(".");
                stdout().flush().unwrap();
            }
            println!("\nDeleted files successfully. \n");
        }
        false => {
            return;
        }
    }
}

pub fn handle_executable(executable: &str) {
    let output = Command::new("which")
        .arg(executable)
        .output()
        .expect("Unable to run the 'which' command!")
        .stdout;
    println!(
        "Executable found: {}",
        String::from_utf8_lossy(&output).trim()
    );
    print!("Remove this executable? (yes, no): ");
    stdout().flush().unwrap();
    match helper::handle_binary_input() {
        true => {
            println!("Deleting file...");
            remove_file(String::from_utf8_lossy(&output).trim())
                .expect("Failed to remove file! Do you have sufficient permission?");
            println!("Deleted file successfully. \n");
        }
        false => {
            return;
        }
    }
}
