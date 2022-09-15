use std::fs::OpenOptions;
use std::io::prelude::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[warn(unused_mut)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn random_user(length: usize) -> String {
    let mut rng = thread_rng();
    let s: String = (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    // println!("{}", s.chars().next().unwrap());
    // print_type_of(&s.chars().next().unwrap());
    let c = s.chars().next().unwrap();
    if c.is_digit(10) {
        return random_user(length);
    }
    return s;
}

fn random_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return password;
}

fn random_email_username_pass(domain: &str, length: usize, amount: i32, file_name: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    for _i in 0..amount {
        let email = random_user(length);
        let username = random_user(length);
        let pass_email = random_password(length);
        let pass_username = random_password(length);

        if let Err(e) = writeln!(
            file,
            "{}@{}\t{}\t{}\t{}",
            email, domain, pass_email, username, pass_username
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn main() {
    random_email_username_pass("proton.me", 8, 100, "listTradingview.txt");
}
