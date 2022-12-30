use std::cell::RefCell;
use std::rc::{Rc, Weak};

use anyhow::Result;
use rand::Rng;

use crate::load::TaskJson;

pub type TaskRef = Rc<Task>;
type WeakTaskRef = Weak<Task>;
type Parent = RefCell<WeakTaskRef>;
type Children = RefCell<Vec<Child>>;
type Child = TaskRef;

pub struct Task {
    pub name: String,
    pub weight: u32,
    pub in_order: bool,
    pub parent: Parent,
    pub children: Children,
}

impl Task {
    fn get_root() -> Result<TaskRef> {
        TaskJson::get()
    }

    fn get_random(self: &TaskRef) -> TaskRef {
        if self.children.borrow().is_empty() {
            return Rc::clone(self);
        }
        let children = self.children.borrow();
        if self.in_order {
            let first_child = Rc::clone(children.first().unwrap());
            return first_child.get_random();
        }

        let mut rng = rand::thread_rng();
        let children = self.children.borrow();
        let max_range = children.iter().map(|c| c.weight).sum();
        let mut rand_weight = rng.gen_range(1..=max_range);

        for child in children.iter() {
            if rand_weight <= child.weight {
                return child.get_random();
            }
            rand_weight -= child.weight;
        }
        unreachable!();
    }

    fn full_name(self: &TaskRef) -> String {
        Rc::clone(self).get_path(&self.name)
    }

    fn get_path(self: TaskRef, name: &str) -> String {
        match self.parent.borrow().upgrade() {
            Some(p) => {
                let parent_name = Self::get_path(Rc::clone(&p), &p.name);
                format!("{}/{}", parent_name, name)
            }
            None => name.to_string()
        }
    }
}

pub fn get_random_task() -> Result<String> {
    let root = Task::get_root()?;
    let task = root.get_random();
    Ok(task.full_name())
}