use std::io::stdin;
use std::collections::HashMap;
use crate::data::Saver;

mod tasks;
mod input;
mod output;
mod data;

fn main() {
    let mut commands = HashMap::<&str, fn() -> ()>::new();
    commands.insert("add habit", handle_add_habit);
    commands.insert("add single", handle_add_single_task);
    commands.insert("add record", handle_add_record);
    commands.insert("check", handle_check);

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let line = line.as_str().trim();
        match commands.get(line) {
            Some(f) => { f() }
            None => match line {
                "exit" => break,
                _ => { println!("Unknown command {line}") },
            },
        }
    }
}

fn handle_add_habit() {
    add_task::<tasks::Habit>();
}

fn handle_add_single_task() {
    add_task::<tasks::SingleTask>();
}

fn add_task<T: tasks::Task + Saver>() {
    input::get_task::<T>().save();
}

fn handle_add_record() {
    input::get_record().save();
}

fn handle_check() {
    let data_habit = data::get_data::<tasks::Habit>();
    let data_single = data::get_data::<tasks::SingleTask>();
}


