use rusqlite::{Connection, Result};

use todo_app::{add_task, delete_task, fetch_tasks, mark_complete};

fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE todo_list (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            is_done BOOL NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

#[test]
fn test_add_and_fetch_task() {
    let conn = setup_test_db().unwrap();
    add_task(&conn, "Test Task", "Do something", false).unwrap();

    let tasks = fetch_tasks(&conn).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].name, "Test Task");
    assert!(!tasks[0].is_done);
}

#[test]
fn test_mark_complete() {
    let conn = setup_test_db().unwrap();
    add_task(&conn, "Mark Me", "To be marked", false).unwrap();
    let task_id = fetch_tasks(&conn).unwrap()[0].id.unwrap();

    mark_complete(&conn, task_id, true).unwrap();

    let tasks = fetch_tasks(&conn).unwrap();
    assert!(tasks[0].is_done);
}

#[test]
fn test_delete_task() {
    let conn = setup_test_db().unwrap();
    add_task(&conn, "Delete Me", "To be deleted", false).unwrap();
    let task_id = fetch_tasks(&conn).unwrap()[0].id.unwrap();

    delete_task(&conn, task_id).unwrap();
    let tasks = fetch_tasks(&conn).unwrap();
    assert_eq!(tasks.len(), 0);
}
