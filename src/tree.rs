use std::cell::RefCell;
use std::rc::{Rc, Weak};

use anyhow::Result;
use rand::Rng;

use crate::tasks::Task;

pub type NodeTaskRef<T> = Rc<NodeTask<T>>;
type WeakNodeNodeRef<T> = Weak<NodeTask<T>>;
type Parent<T> = RefCell<WeakNodeNodeRef<T>>;
type Children<T> = RefCell<Vec<Child<T>>>;
type Child<T> = NodeTaskRef<T>;

pub trait GenericNodeTask {
    fn get_root() -> Result<Self> where Self: Sized;
    fn get_random(&self) -> Self where Self: Sized;
    fn name(&self) -> String;
}

impl<T: Task> GenericNodeTask for NodeTaskRef<T> {
    fn get_root() -> Result<Self> where Self: Sized {
        Ok(T::get_all()?)
    }

    fn get_random(&self) -> NodeTaskRef<T> {
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

    fn name(&self) -> String {
        name_rec(Rc::clone(self), &self.name)
    }
}

fn name_rec<T: Task>(node: NodeTaskRef<T>, name: &str) -> String {
    match node.parent.borrow().upgrade() {
        Some(p) => {
            let parent_name = name_rec(Rc::clone(&p), &p.name);
            format!("{}/{}", parent_name, name)
        }
        None => name.to_string()
    }
}

#[derive(Debug)]
pub struct NodeTask<T: Task> {
    pub task: T,
    pub name: String,
    pub weight: u32,
    pub in_order: bool,
    pub parent: Parent<T>,
    pub children: Children<T>,
}


