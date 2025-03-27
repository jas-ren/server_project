use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let function: &str = &args[1];
    let target_key: &str = &args[2];
    let mut file = File::open("db.txt")?;
    let buf_reader: BufReader<File> = BufReader::new(file);

    match function.to_uppercase().as_str() {
        "GET" => {
            get_by_key(&buf_reader, target_key);
        },
        "SET" => {
            let target_value: &str = &args[3];
            set_value(&mut file, &buf_reader, target_key, target_value);
        }
    }
    Ok(())
}


fn get_by_key(reader: &BufReader<File>, target_key: &str) -> std::io::Result<Option<String>>{
    for line_result in reader.lines() {
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

fn set_value(file: &mut File, reader: &BufReader<File>, target_key: &str, target_value: &str) -> std::io::Result<Option<String>> {

    let mut lines == Vec::new()
    for line_result in reader.lines() {
        let mut line = line_result?; 
        let mut parts = line.splitn(2, " ");
        if let(Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == target_key {
                lines.push(format!("{target_key} {target_value}"));
                found = true;
            }
        }
    }
    file.append(format!("{target_key} {target_value}\n"));
}