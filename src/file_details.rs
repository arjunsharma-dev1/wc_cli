use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use crate::CmdArgManager;

pub struct FileDetails {
    pub contents: String,
    pub file_path: String
}

impl FileDetails {
    pub fn new(contents: String, file_path: String) -> Self {
        FileDetails {
            contents,
            file_path
        }
    }
}

pub fn get_contents_from_files_or_stdin(cmd_arg_manager: &mut CmdArgManager) -> Vec<FileDetails> {
    let mut file_details: Vec<FileDetails> = Vec::new();
    if cmd_arg_manager.is_input_from_stdin() {
        let contents = read_from_stdin();
        file_details.push(FileDetails::new(contents, String::new()));
    } else {
        let mut file_paths = cmd_arg_manager.file_paths_option.as_mut().unwrap();
        while let Some(file_path_option) = file_paths.next() {
            let file_path = file_path_option.to_string();
            let contents = read_from_file(&file_path);
            file_details.push(FileDetails::new(contents, file_path));
        }
    }
    file_details
}

fn read_from_stdin() -> String {
    let mut contents= String::new();
    io::stdin().read_to_string(&mut contents).expect("error reading input from stdin");
    contents
}

fn read_from_file(file_path: &String) -> String  {
    let file = File::open(&file_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents: String = String::new();
    buf_reader.read_to_string(&mut contents).expect("failed to read file");
    contents
}

pub fn get_file_names(file_details: &Vec<FileDetails>) -> Vec<String> {
    file_details
        .iter()
        .map(|file_detail| file_detail.file_path.clone())
        .collect::<Vec<String>>()
}