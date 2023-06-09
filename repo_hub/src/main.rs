use std::env;
use std::path::PathBuf;
use std::process::{Stdio, Command, Output};
use to_vec::ToVec;
use regex::Regex;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len().to_string().parse::<i32>().unwrap() - 1;
    match arg_len{
        0 => get_status(get_repos(get_cwd()), true),
        1.. => parse_args(args),
        _ => {},
    }
}

//name extraction for the repo will not work if it has a slash on it, but whatever.
fn get_status(repos: Vec<String>, simple: bool){
    let re: Regex = Regex::new(r"([^/]+$)").unwrap();
    for path in repos{
        let repo_name: String = re.find(&path).unwrap().as_str().to_string();
        assert!(env::set_current_dir(&path).is_ok());
        let output: Output = Command::new("git").args(["status", "--short"]).stdout(Stdio::piped())
            .output().expect("Not a git Repository!");
        let status: String = String::from_utf8_lossy(&output.stdout).to_string();

        println!("| {}: {}", &repo_name, status_message(status, simple));        
    }
}

fn status_message(m: String, simple: bool) -> String{
    let gb: Output = Command::new("git").args(["branch", "--show-current"]).stdout(Stdio::piped())
        .output().expect("Error!");
    let branch = String::from_utf8_lossy(&gb.stdout).to_string().replace("\n", "");
    match simple {
        true => { return format!("[{}]\n| ?{} | +{} | ~{} | -{} |\n", branch,
                    count_matches(&m, "?? "),
                    count_matches(&m, "A "),
                    count_matches(&m, "M "),
                    count_matches(&m, "D "));
                }
        false => {return format!("[{}]\n{}", branch, get_files_formatted(&m));}
    }
}

fn get_repos(path: PathBuf) -> Vec<String> {
    let dir: String = path.into_os_string().into_string().unwrap();
    let output: Output = Command::new("find")
        .args([&dir,"-name", ".git","-type", "d"])
        .stdout(Stdio::piped())
        .output().expect("Error!");
    let repo_results: String = String::from_utf8_lossy(&output.stdout).to_string()
        .replace("/.git", "");

    repo_results.lines().map(String::from).to_vec()
}

fn get_files_formatted(m: &String) -> String{
    let mut file_list: Vec<(String, String)> = vec![];
    file_list.push(("New".to_string(), get_files_list(&m, Regex::new(r"\?\? (.*)\n").unwrap())));
    file_list.push(("Added".to_string(), get_files_list(&m, Regex::new(r"A (.*)\n").unwrap())));
    file_list.push(("Modified".to_string(), get_files_list(&m, Regex::new(r"M (.*)\n").unwrap())));
    file_list.push(("Deleted".to_string(), get_files_list(&m, Regex::new(r"D (.*)\n").unwrap())));
    
    formatted_list(file_list)
}

fn formatted_list (list: Vec<(String, String)>) -> String{
    let mut final_list: String = "".to_string();
    let mut not_list: Vec<String> = vec![];
    for item in list {
        let i_s = item.1.len().to_string().parse::<i32>().unwrap();
        if i_s > 1 {
            let title: String = format!("| {} Files:\n", item.0).to_string();
            final_list.push_str(&title);
            final_list.push_str(&item.1);
        }
        else {
            not_list.push(item.0);
        }
    }
    let mut final_no_element_list: String = "".to_string();
    let e_s = not_list.len().to_string().parse::<i32>().unwrap();
    match e_s {
        1 => {
            final_no_element_list = format!("{}", not_list.get(0).unwrap()).to_string()},
        2 => {
            final_no_element_list = format!("{} or {}", not_list.get(0).unwrap(), 
            not_list.get(1).unwrap()).to_string()},
        3 => {
            final_no_element_list = format!("{}, {} or {}", not_list.get(0).unwrap(),
            not_list.get(1).unwrap(), not_list.get(2).unwrap()).to_string()},
        4 => {
            final_no_element_list = format!("{}, {}, {} or {}", not_list.get(0).unwrap(), 
            not_list.get(1).unwrap(), not_list.get(2).unwrap(), 
            not_list.get(3).unwrap()).to_string()},
        _ => {}
    }
    if final_no_element_list.is_empty(){
        return final_list;
    }
    else {
        return format!("| No {} Files.\n{}", final_no_element_list, final_list).to_string();
    }
}

fn get_files_list(text: &String, re: Regex) -> String{
    let mut strang: String = "".to_string();
    for cap in re.captures_iter(text){
        let m: String = format!("| _ {}\n", &cap[1]);
        strang.push_str(&m);
    }
    
    strang
}

fn count_matches(text: &String, sub_string: &str) -> String{
    text.matches(&sub_string).count().to_string()
}

fn parse_args(args : Vec<String>){
    for arg in args {
        match arg.as_str(){
            //problem where if other flags are passed, the program doesn't run
            //could solve by creating an args collect
            //___    help   __
            "-h" | "--help"       => {print_help()},
            //___    path   ___
            "-p" | "--path"       => {/* args.next?*/},
            //___   depth   ___
            "-d" | "--depth"      => {},
            //___ expressive___
            "-x" | "--expressive" => {get_status(get_repos(get_cwd()), false)},
            //___   fetch   ___
            "-f" | "--fetch"      => {println!("fetching is not implemented")},
            _ => {},
        }
    }
}

fn print_help(){
    let title: &str =
    "A simple list the status of all git repositories under a directory\n\nOptions:";
    let options1: &str =
    "-h | --help            displays an explanation of the basic functionality and all options";
    let options2: &str =
    "-p | --path [path]     set a specific path to run instead of the current directory";
    let options3: &str =
    "-d | --depth [num]     set a max directory depth for the repositorie search";
    let options4: &str =
    "-x | --expressive      displays a more verbose list of the files staged for commits";
    let options5: &str =
    "-f | --fetch           displays the status the repository if it has new files or branches";
    println!("\n\n\n{}\n\n{}\n{}\n{}\n{}\n{}\n", &title, &options1, &options2, &options3, &options4, &options5);
}

fn get_cwd() -> PathBuf{
    env::current_dir().unwrap()
}
