use std::{cell::{Cell, RefCell}, rc::{Rc, Weak}};


pub struct Task {
    pub title: String,
    pub comleted: Cell<bool>,
    children: RefCell<Vec<Weak<Task>>>,
    parent: RefCell<Option<Weak<Task>>>,
}

impl Task {
    pub fn new(title: impl AsRef<str>) -> Rc<Task> {
        Rc::new(Task {
            title: title.as_ref().to_string(),
            comleted: Cell::new(false),
            children: RefCell::new(vec![]),
            parent: RefCell::new(None),
        })
    }
}

pub trait TaskAPI {
    fn add_subtask(&self, task: Rc<Task>);
    fn mark_completed(&self);
    fn mark_completed_recursive(&self);
    fn print_tree(&self, depth: usize);
}

impl TaskAPI for Rc<Task> {
    fn add_subtask(&self, task: Rc<Task>) {
        
    }

    fn mark_completed(&self) {
        
    }

    fn mark_completed_recursive(&self) {
        
    }

    fn print_tree(&self, depth: usize) {
        
    }
}

fn main() {
    let root = Task::new("Изучить Rust");

    let smart_ptrs = Task::new("Умные указатели");
    let ownership = Task::new("Ownership и Borrowing");

    root.add_subtask(smart_ptrs.clone());
    root.add_subtask(ownership.clone());

    let box_rc = Task::new("Box, Rc, Arc");
    smart_ptrs.add_subtask(box_rc);

    root.print_tree(0);

    smart_ptrs.mark_completed_recursive();

    println!("\nПосле выполнения раздела:");
    root.print_tree(0);
}
