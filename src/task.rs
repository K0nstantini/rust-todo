use std::cell::RefCell;
use std::rc::{Rc, Weak};

use anyhow::Result;

use crate::load::TaskJson;
use crate::random;
use crate::random::RandomItem;

pub type TaskRef = Rc<Task>;
type WeakTaskRef = Weak<Task>;
type Parent = RefCell<WeakTaskRef>;
pub type Children = RefCell<Vec<Child>>;
pub type Child = TaskRef;

pub struct Task {
    pub name: String,
    pub time: u32,
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

        let items = children
            .iter()
            .map(|ch| ch.into())
            .collect();
        let random_item = random::get(items);
        children
            .get(random_item)
            .unwrap()
            .get_random()
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

impl Into<RandomItem> for &Child {
    fn into(self) -> RandomItem {
        RandomItem { weight: self.weight, time: self.time }
    }
}

pub fn get_random_task() -> Result<String> {
    let root = Task::get_root()?;
    let task = root.get_random();
    Ok(task.full_name())
}