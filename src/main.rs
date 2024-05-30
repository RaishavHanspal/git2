use std::process::Command;

fn main() {
    println!("Hello, world!");
    git_clone();
}

fn git_clone() {
    let result = Command::new("git").args(["commit", "-a", "-m", "first commit"]).output();
    println!("res: {:?}", result);
}
