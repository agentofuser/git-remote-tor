use std::env;
use std::process::Command;

fn main() {
    match env::var("GIT_DIR") {
        Ok(val) => {
            Command::new("torsocks")
                .arg("git-remote-http")
                .args(env::args().skip(1))
                .spawn()
                .expect("Error proxying to git-remote-*");
        }
        Err(e) => {
            eprintln!("GIT_DIR is not set: {}", e);
            std::process::exit(1)
        }
    }
}
