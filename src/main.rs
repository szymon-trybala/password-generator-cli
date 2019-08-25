use std::io;

fn main() {
    println!("To generate a password, press enter:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp)
        .expect("Failed to read line, wtf mate");

    println!("Your password is:");
    println!("admin1");
}
