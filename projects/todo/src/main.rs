use std::env;
use todo::command;
use todo::todo_list::TodoList;

fn main() {
    let mut todos = TodoList::load();

    let args: Vec<String> = env::args().collect();

    command::handle_command(&args, &mut todos);
}
