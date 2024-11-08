use clap::Parser;
use rust_search::{similarity_sort, SearchBuilder};
use std::error::Error;
use std::io::{prelude::Write, stdout};
mod handle_files;
mod helper;

#[derive(Parser, Debug)]
#[clap(author = "pine", version, about)]
/// A program to delete programs and related files; This probably shouldn't exist but oh well :)
pub struct Arguments {
    pub program_name: String,
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

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let (display_name, mut desktop_file_path) = helper::display_name_search(&args.program_name);

    if desktop_file_path == "" {
        let mut matching_files: Vec<String> = SearchBuilder::default()
            .location("/usr/share/applications")
            .search_input(&args.program_name.replace(" ", "-"))
            .more_locations(vec!["~/.local/share/applications"])
            .ignore_case()
            .build()
            .collect();

        match matching_files.len() {
            0 => {
                println!("No matches for the program found in application directories!");
                println!("Searching for executables... TODO...");
            }
            1 => {
                println!("Match found: {}", matching_files[0]);
                print!("Confirm deletion of related files? (yes, no): ");
                stdout().flush().unwrap();
                match helper::handle_binary_input() {
                    true => {
                        println!("Understood! \n");
                        desktop_file_path = matching_files[0].to_owned();
                    }
                    false => {
                        println!("Exiting program...");
                        return Ok(());
                    }
                }
            }
            _ => {
                similarity_sort(&mut matching_files, &args.program_name);
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
                        return Ok(());
                    }
                    _ => {
                        println!("Understood! {}\n", &matching_files[selection - 1]);
                        desktop_file_path = matching_files[selection - 1].to_owned();
                    }
                }
            }
        }
    }
    if desktop_file_path != "" {
        if let Err(e) = handle_files::read_desktop_file(&desktop_file_path, &display_name) {
            eprintln!("Application error: {e}");
        }
    }

    // let exclude_string = args.exclude.unwrap_or("".to_string());
    // let exclude_directories: Vec<&str> = exclude_string.rsplit(' ').collect();
    // let base_directory_string = args.base_dir.unwrap_or("".to_string());
    // let base_directories: Vec<&str> = base_directory_string.rsplit(' ').collect();

    // let _search_directories = get_search_directories(exclude_directories, base_directories);

    Ok(())
}
