use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;


pub fn open_file(file_path: &String) -> BufReader<File> {
    let path = Path::new(file_path);

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    BufReader::new(file)
}

pub fn open_file_from_env(error: &str) -> BufReader<File> {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => panic!("{}", error),
    };

    open_file(file_path)
}