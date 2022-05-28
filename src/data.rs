use crate::tasks::{Habit, SingleTask, Record};

pub trait Saver {
    fn save(&self);
}

impl Saver for Habit {
    fn save(&self) {
        unimplemented!();
    }
}

impl Saver for SingleTask {
    fn save(&self) {
        unimplemented!();
    }
}

impl Saver for Record {
    fn save(&self) {
        unimplemented!();
    }
}

pub fn get_data<T: Loader>() -> Vec<(T, Vec<Record>)> {
    T::get_data()
}

pub trait Loader: Sized {
    fn get_data() -> Vec<(Self, Vec<Record>)>;
}

impl Loader for Habit {
    fn get_data() -> Vec<(Self, Vec<Record>)> {
        unimplemented!();
    }
}

impl Loader for SingleTask {
    fn get_data() -> Vec<(Self, Vec<Record>)> {
        unimplemented!();
    }
}
