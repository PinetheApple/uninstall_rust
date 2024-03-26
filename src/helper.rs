use resolve_path::PathResolveExt;
use std::{env, io::stdin};

pub fn handle_binary_input() -> bool {
    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("please enter something");
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

fn path_exists(path: &str) -> bool {
    match path.try_resolve().unwrap().try_exists() {
        Ok(true) => {
            println!("{path}: \t\tExists (0_0)",);
            return true;
        }
        Ok(false) => {
            println!("{path}: \t\tThis path doesn't exist OR isn't a directory")
        }
        Err(_) => {
            println!("{path}: \t\tOops! some issue locating that directory; Perhaps you don't have sufficient permission? Try running this with root privileges.");
        }
    };
    return false;
}

pub fn get_search_directories(
    exclude_directories: Vec<&str>,
    base_directories: Vec<&str>,
) -> Vec<String> {
    let mut search_directories: Vec<String> = Vec::new();
    if base_directories[0] != "" {
        for directory in base_directories {
            if path_exists(directory) {
                search_directories.push(directory.to_string());
            }
        }
        return search_directories;
    }
    let mut path = "".to_string();
    match env::var("PATH") {
        Ok(path_var) => path = path_var,
        Err(e) => print!(
            "Encountered an error trying to read the $PATH environment variable: {}",
            e
        ),
    };
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
