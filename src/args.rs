use clap::Parser;

const TASKS_JSON: &str = "tasks.json";

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = TASKS_JSON.to_string())]
    pub path: String,
}