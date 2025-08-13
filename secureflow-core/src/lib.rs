pub fn run_code(code: &str) {
    for line in code.lines() {
        if line.starts_with("print") {
            let content = line.trim_start_matches("print(").trim_end_matches(");");
            println!("{}", content.trim_matches('"'));
        }
    }
}
