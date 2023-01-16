use chrono::{DateTime, Local};
use encryptfile as ef;
use rand::{distributions::Uniform, Rng};
use std::{fs, io::Write, process::Command, str};

// use bash command in shell with zip enstalled to zip -er files with password,
// not sure how I will enter password. Need to either pass it to the zip command
// or enter it into shell with rust after the first zip command.

fn main() {
    let password: String = generate_password();
    let datetime_string: String = get_datetime_string();
    let full_path: String = get_full_path(datetime_string);
    // write_to_file(&full_path, password);
    write_to_zip(&full_path, &password)
        .unwrap_or_default();
    encrypt_file();
}

fn encrypt_file() {
    let in_file = r#"C:\Users\Kyle\workspace\rust\password_resetter\testin.txt"#;
    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_file.to_owned()))
        .output_stream(ef::OutputStream::File(r#"C:\Users\Kyle\workspace\rust\password_resetter\testout.ef"#.to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .salt("ladslfkaslk4n#$%$#063840")
        .password(ef::PasswordType::Text("iloveyou".to_owned(), ef::scrypt_defaults()))
        .encrypt();
    let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));
}

// fn zip_with_password(filename: &str, password: &str) {
//     Command::new("sh")
//         .arg("-c")
//         .arg(format!("zip -e test.zip test.txt -P {password}"))
//         .output();
// }

fn write_to_zip(filename: &str, content: &str) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    zip.start_file(filename, Default::default())?;
    zip.write_all(content.as_bytes())?;

    zip.finish()?;
    Ok(())
}

fn write_to_file(path: &str, content: String) {
    let error_msg: String = format!("Unable to write password to {path}");
    fs::write(path, content).expect(&error_msg);
}

fn get_full_path(datetime_string: String) -> String {
    let path: String = r#"C:\Users\Kyle\workspace\rust\password_resetter\"#.to_string();
    let full_path: String = format!("{path}{datetime_string}.zip");
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

