use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[clap(author = "Pine", version, about)]
/// A program to delete programs and related files
struct Arguments {
    program_name: String,
    /// search only within a base directory and not using the $PATH variable; for multiple directories, place them in a string separated by a space
    #[arg(short, long, value_name = "BASE_DIR(s)")]
    base_dir: Option<String>,
    /// directory(s) to include in addition to directories present in $PATH variable; for multiple directories, place them in a string separated by a space
    #[arg(short, long, value_name = "directory(s)")]
    include: Option<String>,
    /// directory(s) to exclude from directories present in $PATH variable; for multiple directories, place them in a string separated by a space
    #[arg(short, long, value_name = "directory(s)")]
    exclude: Option<String>,
    /// save configuration files
    #[arg(short, long, num_args = 0)]
    save: bool,
}

fn _is_valid_path(_path: &String) -> bool {
    true
}

fn _path_exists(_path: &String) -> bool {
    true
}

fn _delete_config() {}

fn get_search_directories(
    include_directories: Vec<&str>,
    exclude_directories: Vec<&str>,
    base_directories: Vec<&str>,
) -> Vec<String> {
    let mut path = "".to_string();
    match env::var("PATH") {
        Ok(path_var) => path = path_var,
        Err(e) => print!(
            "Encountered an error trying to read the environment variable: {}",
            e
        ),
    };
    let path_directories: Vec<&str> = path.rsplit(":").collect();
    let mut search_directories: Vec<String> = Vec::new();
    if base_directories[0] != "" {
        for directory in base_directories {
            search_directories.push(directory.to_owned());
        }
        return search_directories;
    }
    search_directories
}

fn main() {
    let args = Arguments::parse();

    let exclude_string = args.exclude.unwrap_or("".to_string());
    let exclude_directories: Vec<&str> = exclude_string.rsplit(' ').collect();
    let include_string = args.include.unwrap_or("".to_string());
    let include_directories: Vec<&str> = include_string.rsplit(' ').collect();
    let base_directory_string = args.base_dir.unwrap_or("".to_string());
    let base_directories: Vec<&str> = base_directory_string.rsplit(' ').collect();
    let search_directories =
        get_search_directories(include_directories, exclude_directories, base_directories);
    println!("{:?}", search_directories);
}
