use clap::{Arg, App};
use std::process;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::str::Chars;

pub struct Config {
    pub length: u32,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_numbers: bool,
    pub use_special: bool,
}

pub enum Type {
    Number,
    LetterUppercase,
    LetterLowercase,
    SpecialCharacter,
}

impl Distribution<Type> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type {
        match rng.gen_range(0, 4) {
            0 => Type::Number,
            1 => Type::LetterUppercase,
            2 => Type::LetterLowercase,
            _ => Type::SpecialCharacter,
        }
    }
}

pub fn get_config() -> Config {
    let mut config = Config {
        length: 16,
        use_uppercase: true,
        use_lowercase: true,
        use_numbers: true,
        use_special: true,
    };

    let matches = App::new("password-generator-cli")
        .version("0.1")
        .author("karmeleonik")
        .about("Very simple, open source app that generate passwords")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("NUMBER")
            .help("Sets the length of your password")
            .takes_value(true))
        .arg(Arg::with_name("exclude")
            .short("x")
            .long("exclude")
            .value_name("CHARS")
            .max_values(3)
            .min_values(1)
            .help("Excludes provided char types - u = uppercase, l = lowercase, n = numbers, s = special. Example: -x lns")
            .takes_value(true))
        .get_matches();

    if let Some(in_file) = matches.value_of("length") {
        match in_file.parse::<u32>() {
            Ok(n) => {
                config.length = n;
            }
            Err(_e) => {
                eprintln!("Couldn't parse argument: --length");
                process::exit(1);
            }
        }
    }
    let iterator = matches.values_of("exclude");
    for el in iterator.unwrap() {
        println!("{:?}", el);
    }

    config
}

pub fn generate_password(cfg: &Config) -> String {
    let LETTERS_UPPERCASE: Chars = "ABCDEFGIJKLMNOPQRSTUVWXYZ".chars();
    let LETTERS_LOWERCASE: Chars = "abcdefghijklmnopqrstuvwxyz".chars();
    let NUMBERS: Chars = "1234567890".chars();
    let SPECIAL_CHARS: Chars = "!@#$%^&*()-+[]<>;,./".chars();

    let mut passwd = String::new();
    let mut random_number_index = rand::thread_rng().gen_range(0, 10);
    let mut random_char_index = rand::thread_rng().gen_range(0, 25);
    let mut random_special_char_index = rand::thread_rng().gen_range(0, 20);

    let mut letters_generated: u32 = 0;
    loop {
        let random_type: Type = rand::random();
        match random_type {
            Type::Number => {
                let mut index: u8 = 0;

                for c in NUMBERS.clone() {
                    if index == random_number_index {
                        passwd.push(c);
                        random_number_index = rand::thread_rng().gen_range(0, 10);
                        break;
                    }
                    index += 1;
                }
            }
            Type::LetterUppercase => {
                let mut index: u8 = 0;

                for c in LETTERS_UPPERCASE.clone() {
                    if index == random_char_index {
                        passwd.push(c);
                        random_char_index = rand::thread_rng().gen_range(0, 25);
                        break;
                    }
                    index += 1;
                }
            }
            Type::LetterLowercase => {
                let mut index: u8 = 0;

                for c in LETTERS_LOWERCASE.clone() {
                    if index == random_char_index {
                        passwd.push(c);
                        random_char_index = rand::thread_rng().gen_range(0, 25);
                        break;
                    }
                    index += 1;
                }
            }
            _ => {
                let mut index: u8 = 0;

                for c in SPECIAL_CHARS.clone() {
                    if index == random_special_char_index {
                        passwd.push(c);
                        random_special_char_index = rand::thread_rng().gen_range(0, 20);
                        break;
                    }
                    index += 1;
                }
            }
        }
        letters_generated += 1;
        if letters_generated > cfg.length - 1 { break; }
    }
    passwd
}