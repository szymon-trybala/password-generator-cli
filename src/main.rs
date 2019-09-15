use pswd::{get_config, generate_password};

fn main() {
    let config = get_config();
    let password = generate_password(&config);
    println!("{}", password);
}

