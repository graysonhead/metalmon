use rpassword::read_password;
use clap::{ArgMatches};


pub fn get_password_or_prompt(matches: &ArgMatches) -> String {
    let pass_input = matches.value_of("password");
    let password = match pass_input {
        None => {
            println!("Type your password:");
            let password = read_password().unwrap();
            password
        }
        Some(p) => {
            p.to_string()
        }
    };
    password
}