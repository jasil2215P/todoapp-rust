use prettytable::{Table, row};
use rusqlite::{Connection, Result, params};

use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

#[derive(Debug)]
struct Task {
    id: Option<usize>,
    name: String,
    description: String,
    is_done: bool,
}

fn fetch_tasks(conn: &Connection) -> Result<Vec<Task>> {
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

fn print_tasks(tasks: Vec<Task>) {
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

fn add_task(conn: &Connection, task: Task) -> Result<usize> {
    conn.execute(
        "INSERT INTO todo_list (name, description, is_done) VALUES (?, ?, ?)",
        params![task.name, task.description, task.is_done],
    )
}

fn delete_task(conn: &Connection, task_id: usize) -> Result<usize> {
    let results = conn.execute("DELETE FROM todo_list WHERE id = ?1", [task_id]);

    let max_id: Option<i64> =
        conn.query_row(&format!("SELECT MAX(id) FROM todo_list"), [], |row| {
            row.get(0)
        })?;

    let max_id = max_id.unwrap_or(0);

    conn.execute(
        "UPDATE sqlite_sequence SET seq = ?1 WHERE name = ?2",
        params![max_id, "todo_list"],
    )?;

    return results;
}

fn mark_complete(conn: &Connection, task_id: usize, completed: bool) -> Result<usize> {
    conn.execute(
        "UPDATE todo_list SET is_done = ? WHERE id = ?",
        params![completed, task_id],
    )
}

fn print_menu() {
    println!();
    println!("TODO-MENU");
    println!("1. List Tasks");
    println!("2. Add Task");
    println!("3. Change 'completed' status");
    println!("4. Delete Task");
    println!("99. Exit");
}

fn main() -> Result<()> {
    let conn: Connection = Connection::open("todo_items.db")?;

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, description TEXT NOT NULL, is_done BOOL NOT NULL)",
        [],
    );

    loop {
        let tasks = fetch_tasks(&conn)?;

        print_menu();
        let input: usize = text_io::read!();
        match input {
            99 => break,
            1 => {
                clear_screen();
                print_tasks(tasks);
            }
            2 => {
                clear_screen();
                print_tasks(tasks);
                let name = read_line("Name of the task> ");
                let description = read_line("Description of the task> ");
                add_task(
                    &conn,
                    Task {
                        id: None,
                        name,
                        description,
                        is_done: false,
                    },
                )?;
            }
            3 => {
                clear_screen();
                print_tasks(tasks);
                let id = read_line("id> ").parse::<usize>().unwrap_or(0);
                mark_complete(&conn, id, true)?;
            }
            4 => {
                clear_screen();
                print_tasks(tasks);
                let id = read_line("id> ").parse::<usize>().unwrap_or(0);
                delete_task(&conn, id)?;
            }
            _ => continue,
        }
    }

    Ok(())
}
