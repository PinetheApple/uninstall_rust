use clap::Parser;

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

fn main() {
    let args = Arguments::parse();
    println!("{}", args.program_name);
    let exclude_string = args.exclude.unwrap_or("".to_string());
    let _exclude_directories: Vec<&str> = exclude_string.rsplit(' ').collect();
    let include_string = args.include.unwrap_or("".to_string());
    let _include_directories: Vec<&str> = include_string.rsplit(' ').collect();
    let base_directory_string = args.base_dir.unwrap_or("".to_string());
    let _base_directories: Vec<&str> = base_directory_string.rsplit(' ').collect();
}
