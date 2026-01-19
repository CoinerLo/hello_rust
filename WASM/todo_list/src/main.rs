

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
