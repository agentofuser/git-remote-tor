use indoc::indoc;
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
        Err(_e) => {
            eprintln!(
                "{}",
                indoc!(
                    "

              Hi there, thanks for checking out git-remote-tor!

              This is a program that is called _by git_ when you use commands
              that interact with remotes over the network.

              So when you call it directly like this, it does nothing (except
              print this message.)

              To use it, add a `tor::` prefix (without the backticks) to the
              remote's url. Here are some examples:

              $ git clone tor::http://3lytcgmoe2j75c6t.onion/ logit

              OR

              $ git remote add agentofuser tor::https://github.com/agentofuser/logit

              After the remote is defined, the `tor::` is saved to `.git/config`,
              so you don't need to think about it again. You can use `git pull`,
              `git fetch`, etc. as you normally would.

              Don't forget to have a tor daemon running and torsocks installed!

              If you find a bug or need help please let me know. More docs
              and contact info are at https://agentofuser.com/git-remote-tor/.
              "
                )
            );
            std::process::exit(1)
        }
    }
}
