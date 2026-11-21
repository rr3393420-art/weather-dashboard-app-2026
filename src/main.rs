mod todo;

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use todo::TodoStore;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "list".to_owned());
    let rest: Vec<String> = args.collect();
    let path = storage_path();
    let mut store = TodoStore::load(&path)?;

    match command.as_str() {
        "add" => {
            let description = rest.join(" ").trim().to_owned();
            if description.is_empty() {
                bail!("Please provide a task description: cargo run -- add \"Buy groceries\"");
            }
            let id = store.add(description)?;
            store.save(&path)?;
            println!("Added task #{id}.");
        }
        "list" | "ls" => {
            if !rest.is_empty() {
                bail!("The list command takes no arguments.");
            }
            store.print_list();
        }
        "done" | "complete" => {
            let id = parse_id(&rest, &command)?;
            store.mark_done(id)?;
            store.save(&path)?;
            println!("Completed task #{id}.");
        }
        "delete" | "remove" | "rm" => {
            let id = parse_id(&rest, &command)?;
            store.delete(id)?;
            store.save(&path)?;
            println!("Deleted task #{id}.");
        }
        "help" | "--help" | "-h" => print_help(),
        other => bail!("Unknown command '{other}'. Run `cargo run -- help` for usage."),
    }

    Ok(())
}

fn storage_path() -> PathBuf {
    std::env::var_os("TODO_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("todos.json"))
}

fn parse_id(args: &[String], command: &str) -> Result<u64> {
    if args.len() != 1 {
        bail!("Usage: cargo run -- {command} <task-id>");
    }
    args[0]
        .parse::<u64>()
        .with_context(|| format!("'{}' is not a valid task ID", args[0]))
}

fn print_help() {
    println!(
        "To-do list\n\n\
         Usage:\n\
         cargo run -- [COMMAND]\n\n\
         Commands:\n\
         add <description>  Add a task\n\
         list               Show all tasks (default)\n\
         done <task-id>     Mark a task as complete\n\
         delete <task-id>   Delete a task\
         help               Show this help\n\n\
         Tasks are saved in todos.json in the current directory. Set TODO_FILE\n\
         to use a different JSON storage file."
    );
}
