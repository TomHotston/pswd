#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::io::stdin;

fn main() {
    let mut stdin_password = String::new();

    let matches = App::new("pswd")
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("pipe")
                .help("Take <PASSWORD> value from stdin, allowing for piped input")
                .short("P")
                .long("pipe"),
        )
        .arg(
            Arg::with_name("PASSWORD")
                .help("The password of which charaters are to be selected from.")
                .required_unless("pipe"),
        )
        .arg(
            Arg::with_name("POSITION")
                .help("The position (integer) of the character to select (starting at 1)")
                .multiple(true),
        )
        .get_matches();

    let mut password = matches.value_of("PASSWORD").unwrap_or("").trim();
    let mut positions = vec![];
    if matches.occurrences_of("POSITION") != 0 {
        positions = values_t_or_exit!(matches, "POSITION", u16);
    }

    if matches.is_present("pipe") {
        if !password.is_empty() {
            positions.insert(0, value_t_or_exit!(matches, "PASSWORD", u16))
        }

        stdin().read_line(&mut stdin_password).expect("Error while reading stdin");
        password = stdin_password.trim();
    }

    match positions.is_empty() {
        true => {
            for (character, index) in password.chars().zip(1..password.len() + 1) {
                println!("{} <- {}", character, index);
            }
        }
        false => {
            for index in positions {
                println!(
                    "{} <- {}",
                    match password.chars().nth((index - 1) as usize) {
                        Some(valid) => valid,
                        None => {
                            println!("The position {} is not in the password.", index - 1);
                            continue;
                        }
                    },
                    index
                    );
            }
        }
    }
}
