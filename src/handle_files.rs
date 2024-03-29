use crate::helper;
use rust_search::SearchBuilder;
use std::io::{prelude::*, stdout};
use std::process::{exit, Command};

pub fn handle_config_files(app_name: &str) {}

pub fn handle_application_files(app_name: &str) {
    let application_files: Vec<String> = SearchBuilder::default()
        .location("/")
        .search_input(app_name)
        .build()
        .collect();

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
                // remove_file(file_name).expect("Failed to remove file! Do you have sufficient permission?");
                println!("whatever here remove this line");
            }
            println!("Deleted files successfully \n");
        }
        false => {
            println!("Exiting program...");
            exit(0);
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
            // remove_file(String::from_utf8_lossy(&output))
            //     .expect("Failed to remove file! Do you have sufficient permission?");
            println!("Deleted file successfully \n");
        }
        false => {
            println!("Exiting program...");
            exit(0);
        }
    }
}
