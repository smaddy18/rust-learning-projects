use crate::todo_list::TodoList;
// enum Command {
//     Add(String),
//     List,
//     Done(usize),
//     Delete(usize),
// }

pub fn handle_command(args: &[String], todos: &mut TodoList) {
    match args[1].as_str() {

        "add" => {
            match args.get(2) {
                Some(task) => todos.add_task(task),
                None => println!("Enter a valid task to do."),
            }
        },

        "list" => todos.list_tasks(),

        "done" => {
            match args.get(2) {
                Some(index) => {
                    if let Ok(i) = index.parse::<usize>() {
                        todos.mark_done(i);
                    }else{
                        println!("Index must be a number");
                    }
                },
                None => println!("Enter a valid index"),
            }
        },

        _ => println!("Unknown command"),
    }
}