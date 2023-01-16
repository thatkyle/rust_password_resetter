use chrono::{DateTime, Local};
use rand::{Rng};
use std::{fs, process::Command, str, iter};

fn main() {
    let acct_password: String = generate_password(12);
    let zip_password: String = generate_password(8);
    let datetime_string: String = get_datetime_string();
    let win_path: String = get_win_path(&datetime_string);
    let win_txt_path: String = format!("{win_path}.txt");
    let sh_txt_path: String = format!("{datetime_string}.txt");
    let zip_path: String = format!("{datetime_string}.zip");
    write_password_to_file(&win_txt_path, acct_password);
    zip_file_with_password(&sh_txt_path, &zip_path, zip_password);
}

fn generate_password(length: usize) -> String {
    let chars: &[u8] = b"#qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM01234567890!@$%^&*()<>[]{}|_+-=";
    let mut rng = rand::thread_rng();
    let char = || chars[rng.gen_range(0..chars.len())] as char;
    let password: String = iter::repeat_with(char).take(length).collect();
    let mut password_chars = password.chars();
    let is_valid_password = password_chars.any(|x| x.is_ascii_digit()) && 
        password_chars.any(|x| x.is_ascii_punctuation()) && 
        password_chars.any(|x| x.is_ascii_lowercase()) && 
        password_chars.any(|x| x.is_ascii_uppercase());
    if is_valid_password {
        password
    } else {
        generate_password(length)
    }
}

// add error handling for Command Output
fn zip_file_with_password(txt_path: &str, zip_path: &str, password: String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("zip -e {zip_path} {txt_path} -P $'{password}'"))
        .output();
    format!("{:#?}", output)
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

fn get_datetime_string() -> String {
    let datetime: DateTime<Local> = chrono::offset::Local::now();
    let datetime_string: String = datetime.to_string();
    let datetime_string_slice: &str = &datetime_string[0..22]
        .replace(" ", "_")
        .replace(":", "-")
        .replace(".", "-");
    datetime_string_slice.to_string()
}

