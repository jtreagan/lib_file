#![allow(dead_code)]
#![allow(unused)]


use std::cell::RefCell;
use std::rc::Rc;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::convert::From;
use lib_file::dir_mngmnt::*;
use lib_file::file_mngmnt::*;
use lib_file::file_fltk::*;

const PROGRAM_TITLE: &str = "File-Management Function Library";
const VERSION: &str = "0.2.0";

fn splash() {
    println!("~~~~~~~~~~~~~~  {}  ~~~~~~~~~~~~~~~~", PROGRAM_TITLE);
    println!("                  VERSION   {}\n\n", VERSION);
}


fn main() {
    let homedirectory = file_get_home_directory();

    println!("\n {} \n", homedirectory);
}




