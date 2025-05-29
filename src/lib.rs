use clap::{ArgAction, Command, arg, command, value_parser};
use prettytable::{Table, row};
use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Task {
    pub id: Option<usize>,
    pub name: String,
    pub description: String,
    pub is_done: bool,
}

pub fn fetch_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, name, description, is_done FROM todo_list")?;

    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            is_done: row.get(3)?,
        })
    })?;

    // Collect into Vec<Task>
    let tasks: Result<Vec<Task>> = task_iter.collect();

    tasks
}

pub fn print_tasks(tasks: Vec<Task>) {
    let mut table: Table = Table::new();

    table.add_row(row!["ID", "Name", "Description", "Completed"]);

    for task in tasks {
        table.add_row(row![
            task.id.unwrap(),
            task.name,
            task.description,
            if task.is_done { "✅" } else { "❌" }
        ]);
    }

    table.printstd();
}

pub fn add_task(conn: &Connection, name: &str, description: &str, is_done: bool) -> Result<usize> {
    conn.execute(
        "INSERT INTO todo_list (name, description, is_done) VALUES (?, ?, ?)",
        params![name, description, is_done],
    )
}

pub fn delete_task(conn: &Connection, task_id: usize) -> Result<usize> {
    let results = conn.execute("DELETE FROM todo_list WHERE id = ?1", [task_id]);

    let max_id: Option<i64> =
        conn.query_row("SELECT MAX(id) FROM todo_list", [], |row| row.get(0))?;

    let max_id = max_id.unwrap_or(0);

    conn.execute(
        "UPDATE sqlite_sequence SET seq = ?1 WHERE name = ?2",
        params![max_id, "todo_list"],
    )?;

    results
}

pub fn mark_complete(conn: &Connection, task_id: usize, completed: bool) -> Result<usize> {
    conn.execute(
        "UPDATE todo_list SET is_done = ? WHERE id = ?",
        params![completed, task_id],
    )
}

pub fn handle_error(result: Result<usize>) {
    match result {
        Ok(_) => {
            println!("[+] Operation executed successfully");
        }
        Err(e) => {
            eprintln!("[-] Operation Failed\nErr: {}", e);
        }
    }
}

pub fn run() -> Result<()> {
    let conn: Connection = Connection::open("todo_items.db")?;

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, description TEXT NOT NULL, is_done BOOL NOT NULL)",
        [],
    );

    let matches = command!()
        .subcommand(Command::new("list").alias("ls").about("List todo items"))
        .subcommand(
            Command::new("add")
                .about("Add a new item")
                .alias("new")
                .arg(arg!([name] "Name of the Task").required(true))
                .arg(arg!([description] "Description of the Task").required(true))
                .arg(
                    arg!(-d --done "Mark the task done")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove an item")
                .aliases(["rm", "del"])
                .arg(
                    arg!([id] "Id of the item to be removed.")
                        .value_parser(value_parser!(usize))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("mark")
                .about("Mark the status of a Task")
                .aliases(["check", "status"])
                .arg(
                    arg!([id] "Id of the item to be marked")
                        .value_parser(value_parser!(usize))
                        .required(true),
                )
                .arg(
                    arg!([status] "Is the task done?")
                        .value_parser(["true", "false"])
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            print_tasks(fetch_tasks(&conn)?);
        }
        Some(("add", cmd)) => {
            let name = cmd
                .get_one::<String>("name")
                .expect("Provided Name is invalid");
            let description = cmd
                .get_one::<String>("description")
                .expect("Provided Description is invalid");

            if name.trim() == "" {
                println!("[!] Name of the task can't be empty, exiting!");
                return Ok(());
            }
            let is_done: bool = *cmd.get_one::<bool>("done").unwrap_or(&false);

            let result = add_task(&conn, name.trim(), description.trim(), is_done);

            handle_error(result);
        }
        Some(("remove", cmd)) => {
            let id: usize = *cmd.get_one::<usize>("id").unwrap();

            let result = delete_task(&conn, id);
            handle_error(result);
        }
        Some(("mark", cmd)) => {
            let id: usize = *cmd.get_one::<usize>("id").unwrap();

            let mark: bool = cmd.get_one::<String>("status").is_none_or(|s| s == "true");

            let result = mark_complete(&conn, id, mark);
            handle_error(result);
        }
        _ => {
            println!("[?] No command found\n[?] Try running `--help` flag.");
        }
    }

    Ok(())
}
