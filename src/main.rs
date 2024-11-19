use std::{path::Path, process::Command};
#[allow(dead_code)]
const URL: &str = "https://github.com/RaishavHanspal/testPrivate.git";
#[allow(dead_code)]
const PATH: &str = "D:/Workspace/misc/privateRepo";
fn main() {
    println!("Hello, world!");
}

fn _open_project(path: &str, url: &str, opts: Vec<&str>) {
    let local_path = Path::new(path);
    if !local_path.exists() {
        println!("local path not present");
        match Command::new("git")
            .args(["clone", url, local_path.to_str().unwrap()])
            .status()
        {
            Ok(_) => {
                _set_local_directory(local_path, opts);
            }
            Err(e) => eprintln!("Error!! {}", e),
        }
    } else {
        _set_local_directory(local_path, opts);
    }
}

fn _set_local_directory(local_path: &Path, opts: Vec<&str>) {
    match std::env::set_current_dir(local_path) {
        Ok(_) => {
            print!("path changed");
            let result = Command::new("git").args(opts).output();
            println!("res: {:?}", result);
        }
        Err(_) => eprint!("Error"),
    };
}

fn _git_command(path: &str, url: &str, command: &str) {
    let opts: Vec<&str> = command.split(' ').collect();
    _open_project(path, url, opts)
}
