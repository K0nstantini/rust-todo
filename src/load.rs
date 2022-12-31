use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use anyhow::{bail, Result};
use serde::Deserialize;

use crate::task::{Task, TaskRef};

const TASKS_JSON: &str = "tasks.json";

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
        let data = match fs::read_to_string(TASKS_JSON) {
            Ok(s) => s,
            Err(e) => bail!("Error reading file '{}': {}", TASKS_JSON, e)
        };
        let task = match serde_json::from_str(&data) {
            Ok(t) => t,
            Err(e) => bail!("Error parsing json '{}': {}", TASKS_JSON, e)
        };
        Ok(task)
    }

    pub fn get() -> Result<TaskRef> {
        let json = Self::load()?;
        Ok(json.into())
    }
}

impl Into<TaskRef> for TaskJson {
    fn into(self) -> TaskRef {
        let parent: Task = (&self).into();
        let parent_ref = Rc::new(parent);
        {
            let mut my_children = parent_ref.children.borrow_mut();

            for child in self.tasks {
                let child_ref = child.into();
                my_children.push(Rc::clone(&child_ref));

                let mut childs_parent = child_ref.parent.borrow_mut();
                *childs_parent = Rc::downgrade(&parent_ref);
            }
        }
        parent_ref
    }
}

impl Into<Task> for &TaskJson {
    fn into(self) -> Task {
        Task {
            name: self.name.clone(),
            time: self.time,
            weight: self.weight,
            in_order: self.in_order,
            parent: RefCell::new(Default::default()),
            children: RefCell::new(vec![]),
        }
    }
}