use anyhow::Result;

use crate::task;

pub fn handle_get() -> Result<()> {
    let task = task::get_random_task()?;
    println!("{task}");
    Ok(())
}
