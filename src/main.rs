use std::{path::Path, process::Command};
const URL: &str = "https://github.com/RaishavHanspal/testPrivate.git";
const PATH: &str = "D:/Workspace/misc/privateRepo";
fn main() {
    println!("Hello, world!");
    git_command(vec!["checkout", "-b", "test1"]);
}

fn open_project(path: &str, url: &str, opts: Vec<&str>) {
    let local_path = Path::new(path);
    if !local_path.exists() {
        println!("local path not present");
        match Command::new("git")
            .args(["clone", url, local_path.to_str().unwrap()])
            .status()
        {
            Ok(_) => {
                set_local_directory(local_path, opts);
            }
            Err(e) => eprintln!("Error!! {}", e),
        }
    } else {
        set_local_directory(local_path, opts);
    }
}

fn set_local_directory(local_path: &Path, opts: Vec<&str>) {
    match std::env::set_current_dir(local_path) {
        Ok(_) => {
            print!("path changed");
            let result = Command::new("git").args(opts).output();
            println!("res: {:?}", result);
        }
        Err(_) => eprint!("Error"),
    };
}

fn git_command(opts: Vec<&str>) {
    open_project(PATH, URL, opts)
}
