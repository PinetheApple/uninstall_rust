use regex::Regex;
use resolve_path::PathResolveExt;
use std::{
    env,
    fs::{read_dir, File},
    io::{prelude::Write, stdin, stdout, Read},
};

pub fn handle_binary_input() -> bool {
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        match input.as_str().trim() {
            "y" | "yes" => {
                return true;
            }
            "n" | "no" | "nein" => {
                return false;
            }
            _ => println!("invalid input, sorry"),
        }
    }
}

pub fn handle_selection_input(length: usize) -> usize {
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read selection!");
        match input.trim().parse::<usize>() {
            Ok(selection) => {
                if selection < length {
                    return selection;
                }
            }
            Err(_) => {}
        }
        println!("Please enter a valid choice!");
    }
}

/// fn to get the program name and desktop file from a potential display name
pub fn display_name_search(display_name: &str) -> (String, String) {
    let home_dir = get_env_variable("home");
    let local_app_dir = home_dir + "/.local/share/applications/";
    let app_dirs = vec!["/usr/share/applications/", local_app_dir.as_str()];
    let name_regex = Regex::new(r"^Name=").unwrap();
    for app_dir in app_dirs {
        let file_names = read_dir(app_dir).expect("Unable to view files in application directory!");
        for file_dir_entry in file_names {
            let res = file_dir_entry.unwrap();
            let desktop_file_name = res
                .file_name()
                .into_string()
                .expect("Couldn't convert into valid String!");
            if !desktop_file_name.ends_with(".desktop") {
                continue;
            }
            let desktop_file_path = String::from(app_dir) + &desktop_file_name;
            let mut desktop_file =
                File::open(&desktop_file_path).expect("Couldn't open desktop file!");

            let mut file_contents = String::new();
            desktop_file
                .read_to_string(&mut file_contents)
                .expect("Unable to read the desktop file into a string");

            for line in file_contents.lines() {
                if !name_regex.is_match(line) {
                    continue;
                }
                let actual_display_name = &line[5..];
                if !actual_display_name.eq_ignore_ascii_case(display_name) {
                    continue;
                }
                println!("Found application with display name \"{actual_display_name}\" at \"{desktop_file_name}\"");
                print!("Is this the correct application? (yes, no): ");
                stdout().flush().unwrap();
                match handle_binary_input() {
                    true => {
                        return (actual_display_name.to_string(), desktop_file_path);
                    }
                    false => {
                        break;
                    }
                }
            }
        }
    }
    return (display_name.to_string(), "".to_string());
}

fn _path_exists(path: &str) -> bool {
    match path.try_resolve().unwrap().try_exists() {
        Ok(true) => {
            println!("{path}: \t\tThe path provided exists");
            return true;
        }
        Ok(false) => {
            println!("{path}: \t\tThis path doesn't exist OR isn't a directory")
        }
        Err(_) => {
            println!("{path}: \t\tUnable to locate that directory; Perhaps you don't have sufficient permission? Try running this with root privileges.");
        }
    };
    return false;
}

fn get_env_variable(var_name: &str) -> String {
    match env::var(var_name.to_ascii_uppercase()) {
        Ok(env_var) => env_var,
        Err(_) => {
            panic!("Encountered an error trying to read the ${var_name} environment variable!");
        }
    }
}

pub fn get_search_directories(
    exclude_directories: Vec<&str>,
    base_directories: Vec<&str>,
) -> Vec<String> {
    let mut search_directories: Vec<String> = Vec::new();
    if base_directories[0] != "" {
        for directory in base_directories {
            if _path_exists(directory) {
                search_directories.push(directory.to_string());
            }
        }
        return search_directories;
    }
    let path = get_env_variable("path");
    let mut path_directories: Vec<&str> = path.rsplit(":").collect();
    if exclude_directories[0] != "" {
        for directory in exclude_directories {
            path_directories.retain(|&x| x != directory);
        }
    }
    for directory in path_directories {
        search_directories.push(directory.to_string());
    }
    return search_directories;
}
