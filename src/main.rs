#![allow(dead_code)]
#![allow(unused)]


/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Notes  ~~~~~~~~~~~~~~~~~~~~~~~~~~


 */

/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Goals  ~~~~~~~~~~~~~~~~~~~~~~~~~~



 */


use lib_file::file_mngmnt::*;
use fltk::*;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::convert::From;
use lib_file::file_fltk::*;

const PROGRAM_TITLE: &str = "File Management Functions";
const VERSION: &str = "0.1.5";

fn splash() {
    println!("~~~~~~~~~~~~~~  {}  ~~~~~~~~~~~~~~~~", PROGRAM_TITLE);
    println!("                  VERSION   {}\n\n", VERSION);
}


fn main() {
    let pathstr = file_nameonly_fltr("/home/jtreagan/programming/rust/mine", "*.lst");

    println!("\n In  main()  the chosen path is:  {} \n", pathstr);
}




