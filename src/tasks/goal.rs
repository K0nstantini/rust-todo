use std::fmt::{Display, Formatter};
use std::fs;
use std::rc::Rc;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::tasks::{NodeTask, Task};
use crate::tree::NodeTaskRef;
use crate::util::default_one;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Goal {}

impl Display for Goal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Task for Goal {
    fn get_all() -> Result<NodeTaskRef<Self>> {
        Ok(JsonGoal::load()?.into())
    }

    fn root_name() -> &'static str {
        "goals"
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonGoal {
    pub name: String,
    #[serde(default = "default_one")]
    pub weight: u32,
    #[serde(default)]
    pub in_order: bool,
    #[serde(default)]
    pub tasks: Vec<JsonGoal>,
}

impl JsonGoal {
    fn load() -> Result<Self> {
        let file_name = Self::json_path();
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

    fn json_path() -> String {
        format!("{}.json", Goal::root_name())
    }
}

impl Into<NodeTaskRef<Goal>> for JsonGoal {
    fn into(self) -> NodeTaskRef<Goal> {
        let parent = NodeTask::new(&self);
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
