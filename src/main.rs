use std::io;
use std::str::FromStr;

use anyhow::{bail, Result};

use crate::tasks::{Goal, SingleTask, Task, TypeTasks};
use crate::tree::{GenericNodeTask, NodeTaskRef};

mod tasks;
mod util;
mod tree;

struct Model<'a> {
    _root: Box<dyn GenericNodeTask + 'a>,
    random_task: Box<dyn GenericNodeTask + 'a>,
}

impl<'a> Model<'a> {
    fn root_and_random<T: Task + 'a>() -> Result<(Box<dyn GenericNodeTask + 'a>, Box<dyn GenericNodeTask + 'a>)> {
        let root = NodeTaskRef::<T>::get_root()?;
        let random_task = root.get_random();
        Ok((Box::new(root), Box::new(random_task)))
    }

    fn new(arg: &str) -> Result<Self> {
        let args: Vec<_> = arg.split(' ').collect();

        let type_task = match (args.get(0), args.get(1)) {
            (Some(_), None) => TypeTasks::get_random_type(),
            (Some(_), Some(&t)) => TypeTasks::from_str(t)?,
            _ => bail!("Invalid 'get' command")
        };


        let (root, random_task) = match type_task {
            TypeTasks::Goal => Self::root_and_random::<Goal>(),
            TypeTasks::Single => Self::root_and_random::<SingleTask>()
        }?;

        let model = Model { _root: root, random_task };
        Ok(model)
    }
}


fn main() -> Result<()> {
    let mut model = None;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let line = line.trim();

        match line.to_lowercase().as_str() {
            a if a.starts_with("get") => {
                model = match Model::new(a) {
                    Ok(m) => {
                        println!("{}", m.random_task.name());
                        Some(m)
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            "done" => match model {
                Some(_) => todo!(),
                None => println!("No current task")
            }
            "exit" => break,
            _ => println!("Unknown command")
        }
    }

    Ok(())
}
