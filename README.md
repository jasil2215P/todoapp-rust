# todoapp-rust

A simple command-line todo application written in Rust using:

- **clap** for CLI argument parsing  
- **rusqlite** for lightweight SQLite database storage  
- **prettytable** for displaying tasks in a clean table format

---

## Features

- Add tasks with name, description, and optional completion status  
- List all tasks in a nice table view  
- Remove tasks by ID  
- Mark tasks as done or not done  
- Persistent storage using SQLite

---

## Requirements

- Rust (tested with Rust 1.87.0)  

---

## Installation & Setup

1. Clone the repo:  
   ```bash
   git clone https://github.com/jasil2215P/todoapp-rust.git
   cd todoapp-rust
   ```
2. Build the project:

```cargo build --release```

3. Run the app with:

```./target/release/todo_app```
Or,
```cargo run -- <ARGS>```


Feel free to open issues or submit PRs if you want to add features or improve the app.
