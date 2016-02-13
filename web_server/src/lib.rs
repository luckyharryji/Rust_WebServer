use std::fs::File;
use std::path::Path;
// use std::net::{TcpListener,TcpStream};
// use std::thread;
use std::io::Result;
// use std::io::BufReader;
use std::io::prelude::*;
// use std::io::ErrorKind;

pub fn get_file_content(path: &Path)->Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
