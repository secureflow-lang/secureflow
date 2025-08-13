use secureflow_core::run_code;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let code = fs::read_to_string(&args[1]).expect("Unable to read file");
        run_code(&code);
    } else {
        println!("SecureFlow REPL v0.0.1");
        println!("Type your code, or 'exit' to quit.");
        let mut input = String::new();
        loop {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            if trimmed == "exit" {
                break;
            }
            run_code(trimmed);
        }
    }
}
