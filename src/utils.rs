use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;


pub fn write_file<P: AsRef<Path>>(path: P, contents: &Vec<u8>) -> io::Result<()> {
    let mut file = try!(File::create(path));
    Ok(try!(file.write_all(contents)))
}


pub fn read_file<P: AsRef<Path>>(path: P, buf: &mut Vec<u8>) -> io::Result<usize> {
    let mut file = try!(File::open(path));
    let bytes = try!(file.read_to_end(buf));
    Ok(bytes)
}


pub fn read_file_to_string<P: AsRef<Path>>(path: P, buf: &mut String) -> io::Result<usize> {
    let mut file = try!(File::open(path));
    let bytes = try!(file.read_to_string(buf));
    Ok(bytes)
}
