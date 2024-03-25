use std::io::stdin;

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
