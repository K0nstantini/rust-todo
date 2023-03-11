use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use anyhow::{bail, Result};
use clap::Parser;
use serde::Deserialize;
use crate::args::Args;

use crate::task::{Task, TaskRef};

pub const fn default_one() -> u32 { 1 }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskJson {
    name: String,
    #[serde(default)]
    time: u32,
    #[serde(default = "default_one")]
    weight: u32,
    #[serde(default)]
    in_order: bool,
    #[serde(default)]
    tasks: Vec<TaskJson>,
}

impl TaskJson {
    fn load() -> Result<Self> {
        let file_name = Args::parse().path;
        let data = match fs::read_to_string(&file_name) {
            Ok(s) => s,
            Err(e) => bail!("Error reading file '{}': {}", file_name, e)
        };
        let task = match serde_json::from_str(&data) {
            Ok(t) => t,
            Err(e) => bail!("Error parsing json '{}': {}", file_name, e)
        };
        Ok(task)
    }

    pub fn get() -> Result<TaskRef> {
        let json = Self::load()?;
        Ok(json.into())
    }
}

impl From<TaskJson> for TaskRef {
    fn from(value: TaskJson) -> Self {
        let parent: Task = (&value).into();
        let parent_ref = Rc::new(parent);
        {
            let mut my_children = parent_ref.children.borrow_mut();

            for child in value.tasks {
                let child_ref = child.into();
                my_children.push(Rc::clone(&child_ref));

                let mut childs_parent = child_ref.parent.borrow_mut();
                *childs_parent = Rc::downgrade(&parent_ref);
            }
        }
        parent_ref
    }
}

impl From<&TaskJson> for Task {
    fn from(value: &TaskJson) -> Self {
        Task {
            name: value.name.clone(),
            time: value.time,
            weight: value.weight,
            in_order: value.in_order,
            parent: RefCell::new(Default::default()),
            children: RefCell::new(vec![]),
        }
    }
}