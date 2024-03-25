use clap::Parser;
use regex::Regex;
use resolve_path::PathResolveExt;
use rust_search::{similarity_sort, SearchBuilder};
use std::process::{exit, Command};
use std::{
    env,
    fs::{remove_file, File},
    io::{prelude::*, stdout, Read},
};
mod helper;

#[derive(Parser, Debug)]
#[clap(author = "Pine", version, about)]
/// A program to delete programs and related files; This probably shouldn't exist but oh well :)
struct Arguments {
    program_name: String,
    // /// search only within a base directory and not using the $PATH variable; for multiple directories, place them in a string separated by a space
    // #[arg(short, long, value_name = "BASE_DIR(s)")]
    // base_dir: Option<String>,
    // /// directory(s) to exclude from directories present in $PATH variable; for multiple directories, place them in a string separated by a space
    // #[arg(short, long, value_name = "directory(s)")]
    // exclude: Option<String>,
    // /// save files in .config directories
    // #[arg(short, long, num_args = 0)]
    // save_config: bool,
    // /// preserve files in application directories and files related to entries present in .desktop files; basically only delete the executable and the desktop entry
    // #[arg(short, long, num_args = 0)]
    // preserve: bool,
    // /// automatically delete relevant files without asking for confirmation; not a great idea
    // #[arg(short, long, num_args = 0)]
    // auto: bool,
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

fn _get_search_directories(
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

fn handle_config_files(app_name: &str) {}

fn handle_application_files(app_name: &str) {
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
                // remove_file(file_name).expect("Failed to remove file");
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

fn handle_executable(executable: &str) {
    let output = Command::new("which")
        .arg(executable)
        .output()
        .expect("Unable to run the 'which' command! Exiting")
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
            //     .expect("Failed to remove file");
            println!("Deleted file successfully \n");
        }
        false => {
            println!("Exiting program...");
            exit(0);
        }
    }
}

fn read_desktop_file(desktop_file_path: &String) {
    let mut desktop_file = File::open(desktop_file_path).expect("Couldn't open that file!");

    let mut file_contents = String::new();
    desktop_file
        .read_to_string(&mut file_contents)
        .expect("Some issue trying to read that file into a string");

    let exec_regex = Regex::new(r"^Exec=").unwrap();
    let icon_regex = Regex::new(r"^Icon=").unwrap();
    let mut executable = "";
    let mut icon = "";

    for line in file_contents.lines() {
        if exec_regex.is_match(line) {
            executable = &line[5..];
        }
        if icon_regex.is_match(line) {
            icon = &line[5..];
        }
    }
    if executable == "" {
        println!("Unable to find executable file!");
        exit(0);
    }
    handle_executable(executable);
    if icon == "" {
        println!("Unable to find icon!");
        exit(0);
    }
    handle_application_files(icon);
    handle_config_files(icon);

    // remove_file(desktop_file_path).expect("Couldn't delete desktop file!");
}

fn main() {
    let args = Arguments::parse();

    let mut matching_files: Vec<String> = SearchBuilder::default()
        .location("/usr/share/applications")
        .search_input(&args.program_name.replace(" ", "-"))
        .more_locations(vec!["~/.local/share/applications"])
        .ignore_case()
        .build()
        .collect();

    similarity_sort(&mut matching_files, &args.program_name);
    match matching_files.len() {
        0 => {
            println!("No matches for the program found in application directories!");
        }
        1 => {
            println!("Match found: {}", matching_files[0]);
            print!("Confirm deletion of related files? (yes, no): ");
            stdout().flush().unwrap();
            match helper::handle_binary_input() {
                true => {
                    println!("Understood! \n");
                    read_desktop_file(&matching_files[0]);
                }
                false => {
                    println!("Aight, aborting");
                    exit(0)
                }
            }
        }
        _ => {
            println!("Multiple matches found: ");
            for i in 0..matching_files.len() {
                println!("{}. {}", i + 1, matching_files[i]);
            }
        }
    }

    // let exclude_string = args.exclude.unwrap_or("".to_string());
    // let exclude_directories: Vec<&str> = exclude_string.rsplit(' ').collect();
    // let base_directory_string = args.base_dir.unwrap_or("".to_string());
    // let base_directories: Vec<&str> = base_directory_string.rsplit(' ').collect();

    // let _search_directories = get_search_directories(exclude_directories, base_directories);
}
