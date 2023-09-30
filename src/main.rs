use clap::{Arg,Command};
use reqwest::{blocking::Client, header, Proxy}; 
use std::fs::File;
use std::io::Read;
use toml::Value;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
fn git_clone(url:&str,dataname:&str){
    let mut git_clone=std::process::Command::new("git");
    let datapath="./data/".to_string()+dataname;
    git_clone.args(&["clone",url,&datapath]);
    let output=git_clone.output().expect("Failed to exec git clone");
    println!("Git clone :{:?}",output);
}

fn read_config()->Vec<String>{
    let mut file = File::open("config.toml").expect("无法打开文件");
    let mut contents=String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let value=contents.parse::<Value>().expect("Filed to parse Toml");
    let mut corpus:Vec<String>=Vec::new();
    if let Some(links)=value["corpus"].as_table(){
        corpus=links
                .keys()
                .filter_map(|f| f.parse::<String>().ok())
                .collect();
    println!("Corpus:{:?}",corpus);
    }
    else{
        println!("Failed to parse Corpus");
    }
    corpus
}
fn copy_file(source_dir:&Path,dest_dir:&str)->std::io::Result<()>{
    let source_file_name = source_dir
        .file_name()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name"))?;
    let destination_path = Path::new(dest_dir).join(source_file_name);
    fs::copy(source_dir,destination_path)?;
    Ok(())
}

fn find_u(filetype:&str){
    let source_dir="./data";
    let dest_dir="./corpus";
    for entry in WalkDir::new(source_dir){
        let entry=entry.unwrap();
        let path=entry.path();
        if let Some(extension)=path.extension(){
            if extension==filetype{
                if let Err(err) = copy_file(path, dest_dir) {
                    println!("Failed to copy file: {:?}", err);
                }
            }
        }
    }
}
fn main() {
    let corpus_links=read_config();
    let mut index=0;
    for i in corpus_links{
        git_clone(&i,&index.to_string());
        index+=1;
    }
    find_u("gif");
}
