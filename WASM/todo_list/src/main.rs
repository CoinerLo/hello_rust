
struct Task<'a> {
    title: String,
    comleted: bool,
    sub_tasks: Vec<&'a Task<'a>>,
    parent_task: Option<&'a Task<'a>>
}

impl Task<'_> {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            comleted: false,
            sub_tasks: vec![],
            parent_task: None
        }
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
