use std::env;
use std::path::PathBuf;
use std::process::{Stdio, Command, Output};
use to_vec::ToVec;
//use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    let arg_len = args.len().to_string().parse::<i32>().unwrap() - 1;
    match arg_len{
        0 => println!("no args"),
        1 => println!("args: {}", &args[1]),
        2 => println!("args: {} - {}", &args[1], &args[2]),
        3 => println!("too many args!"),
        _ => println!("todo"),
    }
    println!("{:?}", get_repos(get_cwd()));
}

fn get_repos(path : PathBuf) -> Vec<String> {
    let dir : String = path.into_os_string().into_string().unwrap();
    let output : Output = Command::new("find")
        .args([&dir,"-name", ".git","-type", "d"])
        .stdout(Stdio::piped())
        .output().expect("Error!");
    let repo_results : String = String::from_utf8_lossy(&output.stdout).to_string();
    let repo_list : Vec<String> = repo_results.lines().map(String::from).to_vec();     
    //repo_list.iter().filter();
    repo_list
}

fn get_cwd() -> PathBuf{
    return env::current_dir().unwrap();
}
