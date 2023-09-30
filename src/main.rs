use clap::{Arg, Command};
use std::fs;
use std::fs::create_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value;
use walkdir::WalkDir;
const COROUS_PATH: &str = "./data";

fn git_clone(url: &str, dataname: &str) {
    let mut git_clone = std::process::Command::new("git");
    let datapath = "./data/".to_string() + dataname;
    git_clone.args(&["clone", url, &datapath]);
    let output = git_clone.output().expect("Failed to exec git clone");
    println!("Git clone :{:?}", output);
}

fn read_config() -> Vec<String> {
    let mut file = File::open("config.toml").expect("无法打开文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let value = contents.parse::<Value>().expect("Filed to parse Toml");
    let mut corpus: Vec<String> = Vec::new();
    if let Some(links) = value["corpus"].as_table() {
        corpus = links
            .keys()
            .filter_map(|f| f.parse::<String>().ok())
            .collect();
        println!("Corpus:{:?}", corpus);
    } else {
        println!("Failed to parse Corpus");
    }
    corpus
}
fn copy_file(source_dir: &Path, dest_dir: &str) -> std::io::Result<()> {
    let source_file_name = source_dir.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name")
    })?;
    let destination_path = Path::new(dest_dir).join(source_file_name);
    fs::copy(source_dir, destination_path)?;
    Ok(())
}

fn find_and_move(filetype: &str, dest_dir: &str) {
    for entry in WalkDir::new(COROUS_PATH) {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == filetype {
                if let Err(err) = copy_file(path, dest_dir) {
                    println!("Failed to copy file: {:?}", err);
                }
            }
        }
    }
}

fn check_path_exist(path: &str) -> bool {
    let path = Path::new(path);
    if path.exists() {
        true
    } else {
        false
    }
}

fn main() {
    let matches = Command::new("Prometheus")
        .arg(
            Arg::new("filetype")
                .short('t')
                .long("filetype")
                .value_name("FILETYPE")
                .required(true)
                .help("The filetype of corpus"),
        )
        .arg(
            Arg::new("location")
                .short('l')
                .long("location")
                .value_name("LOCATION")
                .required(false)
                .help("The location to generate corpus"),
        )
        .get_matches();
    let corpus_links = read_config();
    let mut index = 0;
    for i in corpus_links {
        let path = COROUS_PATH.to_string() + "/" + &index.to_string();
        if !check_path_exist(&path) {
            git_clone(&i, &index.to_string());
        } else {
            println!("Skip the {}", i);
        }
        index += 1;
    }
    let filetype = matches.get_one::<String>("filetype").unwrap();
    let location = if matches.contains_id("location") {
        matches.get_one::<String>("location").unwrap()
    } else {
        "./corpus"
    };

    if !check_path_exist(location) {
        create_dir(Path::new(location)).expect("Failed to create fold");
    }
    find_and_move(filetype, location);
}
