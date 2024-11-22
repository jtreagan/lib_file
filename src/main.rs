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
    let usepath = Rc::new(RefCell::new("/home/jtreagan/programming/rust/mine".to_string()));

    let pathstr = file_nameonly(usepath);

    println!("\n In  main()  the chosen path is:  {} \n", pathstr);
}




