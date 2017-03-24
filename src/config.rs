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
        let temp: Vec<&str> = ln.split(":").collect();
        let name = temp[0];
        let item = temp[1];
        result.set_fileds(name, item);
    }
    Some(result)
}
