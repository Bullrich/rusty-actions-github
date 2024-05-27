use std::{env, io};
use std::fs;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;


#[cfg(windows)]
pub const EOL:&str = "\r\n";
#[cfg(not(windows))]
pub const EOL:&str = "\n";

pub fn issue_file_command(command: &str, message: String) -> Result<(), String> {
    let env_var = format!("GITHUB_{}", command);
    let file_path = match env::var(&env_var) {
        Ok(path) => path,
        Err(_) => return Err(format!("Unable to find environment variable for file command {}", command)),
    };

    if !Path::new(&file_path).exists() {
        return Err(format!("Missing file at path: {}", file_path));
    }

    let mut file = match fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
    {
        Ok(f) => f,
        Err(_) => return Err(format!("Unable to open file at path: {}", file_path)),
    };

    if let Err(_) = writeln!(file, "{}{}", message, EOL) {
        return Err(format!("Unable to write to file at path: {}", file_path));
    }

    Ok(())
}

pub fn prepare_key_value_message(key: &str, value: &str) -> Result<String, String> {
    let delimiter = format!("ghadelimiter_{}", Uuid::new_v4());

    // These should realistically never happen, but just in case someone finds a
    // way to exploit uuid generation let's not allow keys or values that contain
    // the delimiter.
    if key.contains(&delimiter) {
        return Err(format!(
            "Unexpected input: name should not contain the delimiter \"{}\"",
            &delimiter
        ));
    }

    if value.contains(&delimiter) {
        return Err(format!(
            "Unexpected input: value should not contain the delimiter \"{}\"",
            &delimiter
        ));
    }

    Ok(format!("{}<<{}{}{}{}{}", key, delimiter, EOL, value, EOL, delimiter))
}

pub fn issue_old_command(command:&str, name:&str, value: &str) {
    let msg:String = format!("::{} name={}::{}", command, name, value);
    io::stdout().write_all((msg.to_string() + EOL).as_bytes()).expect("Failed to write command");
}