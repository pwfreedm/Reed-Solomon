use std::io::{stdin, stdout, Write, Error};
use std::fs;
use std::string::String;

/** Prompts the user with the provided prompt and returns their input */
pub fn read_line (prompt: &str) -> Result<String, Error>
{
    print!("{}", prompt);
    stdout().flush()?;
    let mut response = String::new();
    stdin().read_line(&mut response)?;
    Ok(response)
}

pub fn get_text_to_encode () -> String
{
    let mut text = read_line("file or text: ").expect("file or text");
    if text.contains("file")
    {
        text = fs::read_to_string(read_line("filepath: ").expect("filepath").trim_end()).expect("valid filepath");
    }
    else if text.contains("text")
    {
        text = read_line("Type the message to encode: ").expect("text");
    }
    text
}
