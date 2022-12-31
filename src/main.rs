use anyhow::Result;

mod util;
mod input;
mod commands;
mod task;
mod load;
mod random;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    input::handle()
}
