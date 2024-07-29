use colored::Colorize;
use std::collections::HashMap;
use std::io;

pub fn confirm(msg: &str) -> bool {
    let allowed_inputs = HashMap::from([("yes", true), ("y", true), ("no", false), ("n", false)]);
    let mut input = String::new();

    while !allowed_inputs.keys().any(|item| &input == item) {
        input.clear();
        println!("{} {}", msg, "Yes(Y) or No(N) only".bright_green());
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        input = String::from(input.to_lowercase().trim());
    }

    allowed_inputs.get(input.as_str()).unwrap().to_owned()
}
