use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;
use std::os::unix::fs::OpenOptionsExt;
use fs2::FileExt;
use nix::unistd::{fork, ForkResult};

const DB_FILE_PATH: &str = "db.txt";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let function: &str = &args[1];
    

    match function.to_uppercase().as_str() {
        "GET" => {
            let target_key: &str = &args[2];
            match get_by_key(target_key)? {
                Some(value) => println!("{value}"),
                None => println!("Key {target_key} not found"),
            }
        },
        "SET" => {
            let target_key: &str = &args[2];
            let target_value: &str = &args[3];
            set_value(target_key, target_value)?;
            println!("Key {target_key} set to {target_value}");
        },
        "DELETE" => {
            let target_keys = &args[2..];
            let mut child_pids = Vec::new();
            let mut failed_forks = Vec::new();


            for key in target_keys {
                match unsafe {
                    fork()
                } {
                    Ok(ForkResult::Parent {child} ) => {
                        child_pids.push(child);
                    },
                    Ok(ForkResult::Child) => {
                        delete_by_key(key)?;
                    }
                    Err(e) => {
                        failed_forks.push((key.to_string(), format!("Fork error: {}", e)));
                    }
                }
            }
        }
        _ => {
            println!("invalid function. Please use GET, SET or DELETE\n");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid function"));
        }
    }
    Ok(())
}


fn get_by_key(target_key: &str) -> std::io::Result<Option<String>>{
    let file = File::open(DB_FILE_PATH)?;
    FileExt::lock_shared(&file)?;
    let bufreader = BufReader::new(&file);

    for line_result in bufreader.lines() {
        let line = line_result?;

        let mut parts = line.splitn(2, " ");
        if let(Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == target_key {
                return Ok(Some(value.to_string()));
            }
        }
    }
    FileExt::unlock(&file)?;
    Ok(None)
}

fn set_value(target_key: &str, target_value: &str) -> std::io::Result<Option<String>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .mode(0o600)
        .open(DB_FILE_PATH)?;
    FileExt::lock_exclusive(&file)?;
    let bufreader = BufReader::new(&file);

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

    let mut bufwriter = BufWriter::new(&mut file);
    for line in lines {
        writeln!(bufwriter, "{line}")?;
    }
    bufwriter.flush()?;
    drop(bufwriter);
    FileExt::unlock(&file)?;
    Ok(None)

}


fn delete_by_key(target_key: &str) -> std::io::Result<Option<String>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(DB_FILE_PATH)?;
    FileExt::lock_exclusive(&file)?;
    let bufreader = BufReader::new(&file);

    let mut lines = Vec::new();
    let mut deleted_key: Option<String> = None;
    for line_result in bufreader.lines() {
        let line = line_result?;
        let mut parts = line.splitn(2, " ");
        if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
            if key != target_key {
                lines.push(line);
            } else {
                deleted_key = Some(val.to_string())
            }
        }
    }

    let mut bufwriter = BufWriter::new(&mut file);
    for line in lines {
        writeln!(bufwriter, "{line}")?;
    }
    bufwriter.flush()?;
    drop(bufwriter);
    FileExt::unlock(&file)?;

    Ok(deleted_key)

}
