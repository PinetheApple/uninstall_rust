use clap::Parser;
use regex::Regex;
use rust_search::{similarity_sort, SearchBuilder};
use std::{
    fs::{remove_file, File},
    io::{prelude::Write, stdout, Read},
};
mod handle_files;
mod helper;

#[derive(Parser, Debug)]
#[clap(author = "BOBby", version, about)]
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

fn read_desktop_file(desktop_file_path: &String, program_name: &str) {
    let mut desktop_file = File::open(desktop_file_path).expect("Couldn't open the desktop file!");

    let mut file_contents = String::new();
    desktop_file
        .read_to_string(&mut file_contents)
        .expect("Some issue trying to read the desktop file into a string");

    let exec_regex = Regex::new(r"^Exec=").unwrap();
    let icon_regex = Regex::new(r"^Icon=").unwrap();
    let name_regex = Regex::new(r"^Name=").unwrap();
    let mut executable = "";
    let mut icon = "";
    let mut name = "";

    for line in file_contents.lines() {
        if exec_regex.is_match(line) {
            executable = &line[5..].split(" ").collect::<Vec<_>>()[0];
        }
        if icon_regex.is_match(line) {
            icon = &line[5..];
        }
        if name_regex.is_match(line) {
            name = &line[5..];
        }
    }
    if executable == "" {
        println!("Unable to find executable file!");
        return;
    }
    handle_files::handle_executable(executable);
    if icon == "" {
        println!("Unable to find icon!");
        return;
    }
    name = if name != "" { name } else { program_name };
    handle_files::handle_application_files(icon);
    handle_files::handle_config_files(name);

    // remove_file(desktop_file_path).expect("Couldn't delete desktop file! Do u have sufficient permission?");
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
            println!("Searching for executables...");
        }
        1 => {
            println!("Match found: {}", matching_files[0]);
            print!("Confirm deletion of related files? (yes, no): ");
            stdout().flush().unwrap();
            match helper::handle_binary_input() {
                true => {
                    println!("Understood! \n");
                    read_desktop_file(&matching_files[0], &args.program_name);
                }
                false => {
                    println!("Exiting program...");
                    return;
                }
            }
        }
        _ => {
            println!("Multiple matches found: ");
            for i in 0..matching_files.len() {
                println!("{}. {}", i + 1, matching_files[i]);
            }
            print!("Enter selection(0 or Ctrl+C to quit): ");
            stdout().flush().unwrap();
            let selection = helper::handle_selection_input(matching_files.len());
            match selection {
                0 => {
                    println!("Exiting program...");
                    return;
                }
                _ => {
                    println!("Understood! {}\n", &matching_files[selection - 1]);
                    read_desktop_file(&matching_files[selection - 1], &args.program_name);
                }
            }
        }
    }

    // let exclude_string = args.exclude.unwrap_or("".to_string());
    // let exclude_directories: Vec<&str> = exclude_string.rsplit(' ').collect();
    // let base_directory_string = args.base_dir.unwrap_or("".to_string());
    // let base_directories: Vec<&str> = base_directory_string.rsplit(' ').collect();

    // let _search_directories = get_search_directories(exclude_directories, base_directories);
}
