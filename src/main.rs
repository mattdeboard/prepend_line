#![feature(use_extern_macros)]
extern crate mktemp;
extern crate regex;
use mktemp::Temp;
use regex::Regex;
use std::{env, fs, io, io::{BufRead, BufReader, Write}, fs::File, path::Path};

fn prepend_file<P: AsRef<Path>>(data: &[u8], file_path: &P) -> io::Result<()> {
  // Create a temporary file
  let mut tmp_path = Temp::new_file()?;
  // Stop the temp file being automatically deleted when the variable
  // is dropped, by releasing it.
  tmp_path.release();
  // Open temp file for writing
  let mut tmp = File::create(&tmp_path)?;
  // Open source file for reading
  let mut src = File::open(&file_path)?;
  // Write the data to prepend
  tmp.write_all(&data)?;
  // Copy the rest of the source file
  io::copy(&mut src, &mut tmp)?;
  fs::remove_file(&file_path)?;
  fs::rename(&tmp_path, &file_path)?;
  Ok(())
}

fn title_string<R>(mut rdr: R) -> String where R: BufRead {
  let mut first_line = String::new();
  rdr.read_line(&mut first_line).expect("Unable to read line");
  first_line
}

fn line_is_flow_annotation(line: String, pattern: &str) -> bool {
  let ptn: Regex = Regex::new(pattern).unwrap();
  ptn.is_match(&line)
}

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let new_file_head = &args[2];
  let ptn = &args[3];
  let file = match fs::File::open(file_path) {
    Ok(file) => file,
    Err(_) => panic!("Unable to read title from {}", file_path),
  };
  let buffer = BufReader::new(file);
  let title = title_string(buffer);

  match line_is_flow_annotation(title, ptn) {
    true => Ok(()),
    false => prepend_file(new_file_head.as_bytes(), &file_path)
  }
}
