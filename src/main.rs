use std::env;

fn main() {
    match env::var("GIT_DIR") {
        Ok(val) => println!("To be implemented. GIT_DIR found: {:?}", val),
        Err(e) => {
            eprintln!("GIT_DIR is not set: {}", e);
            std::process::exit(1)
        }
    }
}
