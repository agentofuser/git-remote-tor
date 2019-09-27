use std::env;
use std::process::Command;

fn main() {
    match env::var("GIT_DIR") {
        Ok(_git_dir) => {
            let args: Vec<String> = env::args().collect();
            let url = &args[2];
            let protocol = url.splitn(2, ':').collect::<Vec<&str>>()[0];
            let remote_helper = "remote-".to_owned() + protocol;

            Command::new("torsocks")
                .arg("git")
                .arg(&remote_helper)
                .args(args.iter().skip(1))
                .spawn()
                .expect(&format!("Error proxying to {}", &remote_helper));
        }
        Err(e) => {
            eprintln!("GIT_DIR is not set: {}", e);
            std::process::exit(1)
        }
    }
}
