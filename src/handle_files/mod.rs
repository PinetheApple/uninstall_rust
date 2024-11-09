use crate::helper::handle_binary_input;
use regex::Regex;
use rust_search::SearchBuilder;
use std::error::Error;
use std::fs;
use std::{
    fs::{remove_dir_all, remove_file},
    io::{prelude::Write, stdout},
    process::Command,
};

pub fn read_desktop_file(
    desktop_file_path: &String,
    program_name: &str,
) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(desktop_file_path)?;

    let exec_regex = Regex::new(r"^Exec=").unwrap();
    let icon_regex = Regex::new(r"^Icon=").unwrap();
    let name_regex = Regex::new(r"^Name=").unwrap();
    let mut executable_name = "";
    let mut icon_name = "";
    let mut display_name = "";

    let _ = file_contents.lines().filter(|line| {
        if exec_regex.is_match(line) {
            executable_name = &line[5..].split(" ").collect::<Vec<_>>()[0];
        }
        if icon_regex.is_match(line) {
            icon_name = &line[5..];
        }
        if name_regex.is_match(line) {
            display_name = &line[5..];
        }
        false
    });

    if executable_name == "" {
        return Err("Failed to find executable file!".into());
    }
    handle_executable(executable_name);

    display_name = if display_name != "" {
        display_name
    } else {
        program_name
    };
    handle_config_files(display_name, program_name, executable_name);

    if icon_name == "" {
        return Err("Failed to find icon!".into());
    }
    handle_application_files(icon_name);

    Ok(())
}

fn handle_config_files(application_name: &str, program_name: &str, executable_name: &str) {
    println!("Searching for configuration directories with the name \"{application_name}\" or \"{program_name}\" or \"{executable_name}\"");
    let mut config_directory: Vec<String> = SearchBuilder::default()
        .location("~/.config")
        .search_input(application_name)
        .depth(1)
        .build()
        .collect();

    if config_directory.len() != 0 {
        delete_config_dir(config_directory);
        return;
    }

    config_directory = SearchBuilder::default()
        .location("~/.config")
        .search_input(program_name)
        .depth(1)
        .build()
        .collect();
    if config_directory.len() != 0 {
        delete_config_dir(config_directory);
        return;
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
    delete_config_dir(config_directory);
}

fn handle_application_files(app_name: &str) {
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
    match handle_binary_input() {
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

fn handle_executable(executable: &str) {
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
    match handle_binary_input() {
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

fn delete_config_dir(config_directory: Vec<String>) {
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
