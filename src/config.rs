use model::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn parse_config(path: String) -> Option<Config> {
    let mut result = Config::default();

    let buf: BufReader<File> = match File::open(path.clone()) {
        Ok(f) => BufReader::new(f),
        Err(_) => return None,
    };

    let lines = buf.lines().filter(|x| x.is_ok()).map(|x| x.unwrap());
    for ln in lines {
        if let Some(split_index) = ln.find(":") {
            let (name, item) = ln.split_at(split_index);
            result.set_fileds(name, item);
        }
    }
    Some(result)
}
