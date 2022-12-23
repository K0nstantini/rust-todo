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
    current_task: Box<dyn GenericNodeTask + 'a>,
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


        let (root, current_task) = match type_task {
            TypeTasks::Goal => Self::root_and_random::<Goal>(),
            TypeTasks::Single => Self::root_and_random::<SingleTask>()
        }?;

        let model = Model { _root: root, current_task };
        Ok(model)
    }
}


fn main() -> Result<()> {
    let mut model = None;

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        match line.trim().to_lowercase().as_str() {
            a if a.starts_with("get") => {
                model = match Model::new(a) {
                    Ok(m) => {
                        println!("{}", m.current_task.name());
                        Some(m)
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            "done" => match model {
                Some(ref m) => println!("Task '{}' completed and deleted", m.current_task.name()),
                None => println!("No current task")
            }
            "exit" => match model {
                Some(ref m) => println!("Task '{}' is running", m.current_task.name()),
                None => break
            },
            _ => println!("Unknown command")
        }
    }

    Ok(())
}
