fn main() {
    if let Err(err) = planner_http::run() {
        eprintln!("Error running http server: {err}");
    }
}
