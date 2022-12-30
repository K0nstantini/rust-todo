use std::io;

use anyhow::Result;

use crate::commands;

pub fn handle() -> Result<()> {
    loop {
        match read_line()?.as_str() {
            "get" => commands::handle_get()?,
            "exit" => break,
            _ => println!("Unknown command")
        }
    }
    Ok(())
}

fn read_line() -> Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let line = line.trim().to_lowercase();
    Ok(line)
}
