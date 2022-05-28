use chrono::{DateTime, Local};

pub trait Task {}

pub struct Habit {}

impl Task for Habit {}

pub struct SingleTask {}

impl Task for SingleTask {}

pub struct Record {}

pub struct TaskInfo {
    task_name: String,
    deadline: Option<DateTime<Local>>,
}
