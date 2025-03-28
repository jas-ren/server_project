use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;

const DB_FILE_PATH: &str = "db.txt";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let function: &str = &args[1];
    let target_key: &str = &args[2];

    match function.to_uppercase().as_str() {
        "GET" => {
            match get_by_key(target_key)? {
                Some(value) => println!("{value}"),
                None => println!("Key {target_key} not found"),
            }
        },
        "SET" => {
            let target_value: &str = &args[3];
            set_value(target_key, target_value)?;
            println!("Key {target_key} set to {target_value}");
        },
        _ => {
            println!("invalid function. Please use GET or SET\n");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid function"));
        }
    }
    Ok(())
}


fn get_by_key(target_key: &str) -> std::io::Result<Option<String>>{
    let file = File::open(DB_FILE_PATH)?;
    let bufreader = BufReader::new(file);

    for line_result in bufreader.lines() {
        let line = line_result?;

        let mut parts = line.splitn(2, " ");
        if let(Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == target_key {
                return Ok(Some(value.to_string()));
            }
        }
    }
    Ok(None)
}

fn set_value(target_key: &str, target_value: &str) -> std::io::Result<Option<String>> {
    let file = File::open(DB_FILE_PATH)?;
    let bufreader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut found = false;

    for line_result in bufreader.lines() {
        let line = line_result?; 
        let mut parts = line.splitn(2, " ");
        if let(Some(key), Some(_)) = (parts.next(), parts.next()) {
            if key == target_key {
                lines.push(format!("{target_key} {target_value}"));
                found = true;
            } else {
                lines.push(line);
            }
        }
    }

    if !found {
        lines.push(format!("{target_key} {target_value}"));
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(DB_FILE_PATH)?;

    let mut bufwriter = BufWriter::new(file);
    for line in lines {
        writeln!(bufwriter, "{line}")?;
    }
    Ok(None)

}