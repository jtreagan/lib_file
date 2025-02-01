#![allow(dead_code)]
#![allow(unused)]


/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Notes  ~~~~~~~~~~~~~~~~~~~~~~~~~~


 */

/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Goals  ~~~~~~~~~~~~~~~~~~~~~~~~~~



 */

use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::convert::From;
//use fltk::*;
use lib_file::file_mngmnt::*;
use lib_file::file_fltk::*;

const PROGRAM_TITLE: &str = "File Management Function Library";
const VERSION: &str = "0.2.0";

fn splash() {
    println!("~~~~~~~~~~~~~~  {}  ~~~~~~~~~~~~~~~~", PROGRAM_TITLE);
    println!("                  VERSION   {}\n\n", VERSION);
}


fn main() {
    let filename = "/home/jtreagan/programming/rust/mine/tr_rbld1/David_config.yaml";

    match file_read_file_to_string(filename) {
        Ok(contents) => {
            println!("\n The file contents is:\n{} \n", contents);
        }
        Err(err) => {
            eprintln!("\n Error reading the file: {} \n", err);
        }
    }
}




