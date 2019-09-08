use rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rand::seq::index::sample;

fn main() {
    let letters_uppercase = "ABCDEFGHIJKLMNOPQRSTUWXYZ".chars();
    let letters_lowercase = "abcdefghijklmnopqrstuwxyz".chars();
    let numbers = "1234567890".chars();
    let special_characters = "!@#$%^&*()-+[]<>;,./".chars();

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

                for c in numbers.clone() {
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

                for c in letters_uppercase.clone() {
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

                for c in letters_lowercase.clone() {
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

                for c in special_characters.clone() {
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
        if letters_generated > 8 { break; }
    }
    println!("{}", passwd);
}

enum Type {
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