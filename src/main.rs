fn main() {
    if let Err(e) = todo_app::run() {
        eprintln!("[-] Error : {}", e);
    }
}
