use chrono::{DateTime, Local};
use rand::{Rng};
use std::{fs, process::Command, str, iter};
use regex::Regex;

// only remaining issue is to solve EOF errors, can either adjust character set or sh command
fn main() {
    let acct_password: String = generate_password(12);
    let zip_password: String = generate_password(12);
    let datetime_string: String = get_datetime_string();
    let win_path: String = get_win_path(&datetime_string);
    let win_txt_path: String = format!("{win_path}.txt");
    let sh_txt_path: String = format!("{datetime_string}.txt");
    let zip_path: String = format!("{datetime_string}.zip");
    println!("{}", zip_path);
    println!("{}", zip_password);
    write_password_to_file(&win_txt_path, acct_password);
    zip_file_with_password(&sh_txt_path, &zip_path, zip_password)
}

fn generate_password(length: usize) -> String {
    let chars: &[u8] = b"#qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM01234567890@$%^&*";
    let mut rng = rand::thread_rng();
    let char = || chars[rng.gen_range(0..chars.len())] as char;
    let password: String = iter::repeat_with(char).take(length).collect();

    let lowercase_regex = Regex::new(r"[a-z]").unwrap();
    let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
    let number_regex = Regex::new(r"[0-9]").unwrap();
    let symbol_regex = Regex::new(r"[@#$%^&*]").unwrap();
    if lowercase_regex.is_match(&password) &&
        uppercase_regex.is_match(&password) &&
        number_regex.is_match(&password) &&
        symbol_regex.is_match(&password) {
        password
    } else {
        generate_password(length)
    }
}

fn zip_file_with_password(txt_path: &str, zip_path: &str, password: String) {
    println!("{} {} {}", txt_path, zip_path, password);
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("zip -e {zip_path} {txt_path} -P {password}"))
        .output();
    println!("{:#?}", output)
}

fn write_password_to_file(path: &str, password: String) {
    let error_msg: String = format!("Unable to write password to {path}");
    fs::write(path, password).expect(&error_msg);
}

fn get_win_path(datetime_string: &str) -> String {
    let path: String = r#"C:\Users\Kyle\workspace\rust\password_resetter\"#.to_string();
    let full_path: String = format!("{path}{datetime_string}");
    full_path
}

// fn generate_password() -> String {
//     let password: String = rand::thread_rng()
//         // .sample_iter(Uniform::new(char::from(35), char::from(126)))
//         .sample_iter(Uniform::from("abcdefg"))
//         .take(12)
//         .map(char::from)
//         .collect();
//     format!("{}\n", password)
// }

fn get_datetime_string() -> String {
    let datetime: DateTime<Local> = chrono::offset::Local::now();
    let datetime_string: String = datetime.to_string();
    let datetime_string_slice: &str = &datetime_string[0..22]
        .replace(" ", "_")
        .replace(":", "-")
        .replace(".", "-");
    datetime_string_slice.to_string()
}

