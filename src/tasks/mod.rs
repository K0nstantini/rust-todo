use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::{Weak};
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use enum_iterator::Sequence;
use rand::Rng;

pub use goal::*;
pub use record::Record;
pub use single_task::SingleTask;
use crate::tree::{NodeTask, NodeTaskRef};

mod goal;
mod habit;
mod just_do_it;
mod record;
mod single_task;


pub trait Task: Display + Default + Debug {
    fn get_all() -> Result<NodeTaskRef<Self>>;
    fn root_name() -> &'static str;
}


impl<T: Task> NodeTask<T> {
    pub fn new(value: &JsonGoal) -> Self {
        NodeTask {
            task: T::default(),
            name: value.name.clone(),
            weight: value.weight,
            in_order: value.in_order,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        }
    }

    // pub fn add_child_and_update_its_parent(&mut self, child: Self) {
    //     let mut my_children = self.children.borrow_mut();
    //     let child_ref = Rc::new(child);
    //     my_children.push(Rc::clone(&child_ref));
    //
    //     let mut childs_parent = child.parent.borrow_mut();
    //     let parent_ref = Rc::new(*self);
    //     *childs_parent = Rc::downgrade(&parent_ref);
    // }

    fn save(&self) -> Result<()> {
        // let root = self.root();
        // let json = serde_json::to_string(root)?;
        //
        // let file_name = Self::json_path();
        // fs::write(file_name, json)?;

        Ok(())
    }


}

#[derive(Copy, Clone, Sequence)]
pub enum TypeTasks {
    Goal,
    Single,
}

impl TypeTasks {
    fn all() -> Vec<Self> {
        enum_iterator::all().collect()
    }

    pub fn get_random_type() -> Self {
        let tasks = Self::all();
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0..tasks.len());
        tasks[rand_idx]
    }

    // pub fn get_random_task(&self) -> Result<Box<dyn GenericNodeTask>> {
    //     let task: Box<dyn GenericNodeTask> = match self {
    //         TypeTasks::Goal => Box::new(NodeTask::<Goal>::get_random()?),
    //         TypeTasks::Single => Box::new(NodeTask::<SingleTask>::get_random()?)
    //     };
    //     Ok(task)
    // }
}

impl FromStr for TypeTasks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let task = match s.to_lowercase().as_str() {
            s if s == Goal::root_name() => Self::Goal,
            s if s == SingleTask::root_name() => Self::Single,
            _ => bail!("Unknown type task")
        };
        Ok(task)
    }
}

const fn default_one() -> u32 { 1 }