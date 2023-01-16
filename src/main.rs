use chrono::{DateTime, Local};
use rand::{distributions::Uniform, Rng};
use std::{fs, process::Command, str};

// only remaining issue is to solve EOF errors, can either adjust character set or sh command
fn main() {
    let acct_password: String = generate_password();
    let zip_password: String = generate_password();
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

fn generate_password() -> String {
    let password: String = rand::thread_rng()
        .sample_iter(Uniform::new(char::from(32), char::from(126)))
        .take(12)
        .map(char::from)
        .collect();
    password
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

